#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Media_Capture_Core")]
pub mod Core;
#[cfg(feature = "Media_Capture_Frames")]
pub mod Frames;
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AdvancedCapturedPhoto(::windows::core::IUnknown);
impl AdvancedCapturedPhoto {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Frame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn Mode(&self) -> ::windows::core::Result<super::Devices::AdvancedPhotoMode> {
        let this = self;
        unsafe {
            let mut result__: super::Devices::AdvancedPhotoMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::AdvancedPhotoMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Context(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Context)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameBoundsRelativeToReferencePhoto(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>> {
        let this = &::windows::core::Interface::cast::<IAdvancedCapturedPhoto2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameBoundsRelativeToReferencePhoto)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::Rect>>(result__)
        }
    }
}
impl ::core::clone::Clone for AdvancedCapturedPhoto {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AdvancedCapturedPhoto {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedCapturedPhoto;{f072728b-b292-4491-9d41-99807a550bbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_Vtbl;
    const IID: ::windows::core::GUID = <IAdvancedCapturedPhoto as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdvancedCapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedCapturedPhoto";
}
impl ::core::convert::From<AdvancedCapturedPhoto> for ::windows::core::IUnknown {
    fn from(value: AdvancedCapturedPhoto) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedCapturedPhoto> for ::windows::core::IUnknown {
    fn from(value: &AdvancedCapturedPhoto) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdvancedCapturedPhoto> for ::windows::core::IInspectable {
    fn from(value: AdvancedCapturedPhoto) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedCapturedPhoto> for ::windows::core::IInspectable {
    fn from(value: &AdvancedCapturedPhoto) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdvancedCapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdvancedCapturedPhoto {}
unsafe impl ::core::marker::Sync for AdvancedCapturedPhoto {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AdvancedPhotoCapture(::windows::core::IUnknown);
impl AdvancedPhotoCapture {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureWithContextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, context: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureWithContextAsync)(::core::mem::transmute_copy(this), context.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AdvancedCapturedPhoto>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionalReferencePhotoCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, OptionalReferencePhotoCapturedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OptionalReferencePhotoCaptured)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionalReferencePhotoCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOptionalReferencePhotoCaptured)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AllPhotosCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AdvancedPhotoCapture, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllPhotosCaptured)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAllPhotosCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAllPhotosCaptured)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FinishAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AdvancedPhotoCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AdvancedPhotoCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AdvancedPhotoCapture;{83ffaafa-6667-44dc-973c-a6bce596aa0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_Vtbl;
    const IID: ::windows::core::GUID = <IAdvancedPhotoCapture as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AdvancedPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.AdvancedPhotoCapture";
}
impl ::core::convert::From<AdvancedPhotoCapture> for ::windows::core::IUnknown {
    fn from(value: AdvancedPhotoCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedPhotoCapture> for ::windows::core::IUnknown {
    fn from(value: &AdvancedPhotoCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AdvancedPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AdvancedPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AdvancedPhotoCapture> for ::windows::core::IInspectable {
    fn from(value: AdvancedPhotoCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AdvancedPhotoCapture> for ::windows::core::IInspectable {
    fn from(value: &AdvancedPhotoCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AdvancedPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AdvancedPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AdvancedPhotoCapture {}
unsafe impl ::core::marker::Sync for AdvancedPhotoCapture {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastBackgroundService(::windows::core::IUnknown);
impl AppBroadcastBackgroundService {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetPlugInState(&self, value: AppBroadcastPlugInState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPlugInState)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPlugInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlugInState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetSignInInfo<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastBackgroundServiceSignInInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSignInInfo)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SignInInfo(&self) -> ::windows::core::Result<AppBroadcastBackgroundServiceSignInInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignInInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastBackgroundServiceSignInInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetStreamInfo<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastBackgroundServiceStreamInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStreamInfo)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StreamInfo(&self) -> ::windows::core::Result<AppBroadcastBackgroundServiceStreamInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastBackgroundServiceStreamInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn BroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BroadcastTitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetViewerCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetViewerCount)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ViewerCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewerCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TerminateBroadcast(&self, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).TerminateBroadcast)(::core::mem::transmute_copy(this), reason, providerspecificreason).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeartbeatRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, AppBroadcastHeartbeatRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HeartbeatRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHeartbeatRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHeartbeatRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TitleId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetBroadcastTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBroadcastTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn BroadcastLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BroadcastLanguage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetBroadcastLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBroadcastLanguage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn BroadcastChannel(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BroadcastChannel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetBroadcastChannel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBroadcastChannel)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BroadcastTitleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BroadcastTitleChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBroadcastTitleChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBroadcastTitleChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BroadcastLanguageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BroadcastLanguageChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBroadcastLanguageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBroadcastLanguageChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BroadcastChannelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundService, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BroadcastChannelChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBroadcastChannelChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundService2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBroadcastChannelChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastBackgroundService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastBackgroundService {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundService;{bad1e72a-fa94-46f9-95fc-d71511cda70b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastBackgroundService as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastBackgroundService {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundService";
}
impl ::core::convert::From<AppBroadcastBackgroundService> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastBackgroundService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundService> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastBackgroundService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastBackgroundService> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastBackgroundService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundService> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastBackgroundService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastBackgroundService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceSignInInfo(::windows::core::IUnknown);
impl AppBroadcastBackgroundServiceSignInInfo {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignInState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOAuthRequestUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOAuthRequestUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OAuthRequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OAuthRequestUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetOAuthCallbackUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOAuthCallbackUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OAuthCallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OAuthCallbackUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationResult)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetUserName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUserName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignInStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, AppBroadcastSignInStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignInStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSignInStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSignInStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UserNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceSignInInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserNameChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUserNameChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundServiceSignInInfo2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUserNameChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastBackgroundServiceSignInInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastBackgroundServiceSignInInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo;{5e735275-88c8-4eca-89ba-4825985db880})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastBackgroundServiceSignInInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastBackgroundServiceSignInInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceSignInInfo";
}
impl ::core::convert::From<AppBroadcastBackgroundServiceSignInInfo> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastBackgroundServiceSignInInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceSignInInfo> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastBackgroundServiceSignInInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastBackgroundServiceSignInInfo> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastBackgroundServiceSignInInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceSignInInfo> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastBackgroundServiceSignInInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastBackgroundServiceSignInInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastBackgroundServiceStreamInfo(::windows::core::IUnknown);
impl AppBroadcastBackgroundServiceStreamInfo {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastStreamState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetDesiredVideoEncodingBitrate(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDesiredVideoEncodingBitrate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn DesiredVideoEncodingBitrate(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DesiredVideoEncodingBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetBandwidthTestBitrate(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBandwidthTestBitrate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn BandwidthTestBitrate(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BandwidthTestBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetAudioCodec<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioCodec)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioCodec(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioCodec)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn BroadcastStreamReader(&self) -> ::windows::core::Result<AppBroadcastStreamReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BroadcastStreamReader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamReader>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, AppBroadcastStreamStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStreamStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoEncodingResolutionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoEncodingResolutionChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoEncodingResolutionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVideoEncodingResolutionChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoEncodingBitrateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastBackgroundServiceStreamInfo, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoEncodingBitrateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoEncodingBitrateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVideoEncodingBitrateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ReportProblemWithStream(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppBroadcastBackgroundServiceStreamInfo2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportProblemWithStream)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastBackgroundServiceStreamInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastBackgroundServiceStreamInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo;{31dc02bc-990a-4904-aa96-fe364381f136})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastBackgroundServiceStreamInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastBackgroundServiceStreamInfo {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastBackgroundServiceStreamInfo";
}
impl ::core::convert::From<AppBroadcastBackgroundServiceStreamInfo> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastBackgroundServiceStreamInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceStreamInfo> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastBackgroundServiceStreamInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastBackgroundServiceStreamInfo> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastBackgroundServiceStreamInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastBackgroundServiceStreamInfo> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastBackgroundServiceStreamInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastBackgroundServiceStreamInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastCameraCaptureState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastCameraCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraCaptureState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastCameraCaptureState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraCaptureState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastCameraCaptureStateChangedEventArgs(::windows::core::IUnknown);
impl AppBroadcastCameraCaptureStateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn State(&self) -> ::windows::core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastCameraCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs;{1e334cd0-b882-4b88-8692-05999aceb70f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastCameraCaptureStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastCameraCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastCameraCaptureStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastCameraCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastCameraCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastCameraCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastCameraCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastCameraCaptureStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastCameraOverlayLocation {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastCameraOverlayLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraOverlayLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastCameraOverlayLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlayLocation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastCameraOverlaySize {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastCameraOverlaySize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCameraOverlaySize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastCameraOverlaySize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCameraOverlaySize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastCaptureTargetType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastCaptureTargetType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastCaptureTargetType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastCaptureTargetType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastCaptureTargetType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastExitBroadcastModeReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastExitBroadcastModeReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastExitBroadcastModeReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastExitBroadcastModeReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastExitBroadcastModeReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastGlobalSettings(::windows::core::IUnknown);
impl AppBroadcastGlobalSettings {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsBroadcastEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBroadcastEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsDisabledByPolicy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisabledByPolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsGpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGpuConstrained)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn HasHardwareEncoder(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasHardwareEncoder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAudioCaptureEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsAudioCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAudioCaptureEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsMicrophoneCaptureEnabledByDefault)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMicrophoneCaptureEnabledByDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEchoCancellationEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsEchoCancellationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEchoCancellationEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSystemAudioGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SystemAudioGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemAudioGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMicrophoneGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MicrophoneGain(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicrophoneGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsCameraCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCameraCaptureEnabledByDefault)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsCameraCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCameraCaptureEnabledByDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetSelectedCameraId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSelectedCameraId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SelectedCameraId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectedCameraId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetCameraOverlayLocation(&self, value: AppBroadcastCameraOverlayLocation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCameraOverlayLocation)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CameraOverlayLocation(&self) -> ::windows::core::Result<AppBroadcastCameraOverlayLocation> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraOverlayLocation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraOverlayLocation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraOverlayLocation>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetCameraOverlaySize(&self, value: AppBroadcastCameraOverlaySize) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCameraOverlaySize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CameraOverlaySize(&self) -> ::windows::core::Result<AppBroadcastCameraOverlaySize> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraOverlaySize = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraOverlaySize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraOverlaySize>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCursorImageCaptureEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCursorImageCaptureEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastGlobalSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastGlobalSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastGlobalSettings;{b2cb27a5-70fc-4e17-80bd-6ba0fd3ff3a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastGlobalSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastGlobalSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastGlobalSettings";
}
impl ::core::convert::From<AppBroadcastGlobalSettings> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastGlobalSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastGlobalSettings> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastGlobalSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastGlobalSettings> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastGlobalSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastGlobalSettings> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastGlobalSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastGlobalSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastHeartbeatRequestedEventArgs(::windows::core::IUnknown);
impl AppBroadcastHeartbeatRequestedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastHeartbeatRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastHeartbeatRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs;{cea54283-ee51-4dbf-9472-79a9ed4e2165})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastHeartbeatRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastHeartbeatRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastHeartbeatRequestedEventArgs";
}
impl ::core::convert::From<AppBroadcastHeartbeatRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastHeartbeatRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastHeartbeatRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastHeartbeatRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastHeartbeatRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastHeartbeatRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
pub struct AppBroadcastManager {}
impl AppBroadcastManager {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetGlobalSettings() -> ::windows::core::Result<AppBroadcastGlobalSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetGlobalSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastGlobalSettings>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ApplyGlobalSettings<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastGlobalSettings>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ApplyGlobalSettings)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetProviderSettings() -> ::windows::core::Result<AppBroadcastProviderSettings> {
        Self::IAppBroadcastManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetProviderSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastProviderSettings>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ApplyProviderSettings<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastProviderSettings>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IAppBroadcastManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ApplyProviderSettings)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastManagerStatics<R, F: FnOnce(&IAppBroadcastManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppBroadcastManager, IAppBroadcastManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppBroadcastManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastManager";
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastMicrophoneCaptureState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastMicrophoneCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastMicrophoneCaptureState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastMicrophoneCaptureState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastMicrophoneCaptureState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastMicrophoneCaptureStateChangedEventArgs(::windows::core::IUnknown);
impl AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn State(&self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastMicrophoneCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastMicrophoneCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs;{a86ad5e9-9440-4908-9d09-65b7e315d795})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastMicrophoneCaptureStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastMicrophoneCaptureStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastMicrophoneCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastMicrophoneCaptureStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPlugIn(::windows::core::IUnknown);
impl AppBroadcastPlugIn {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ProviderSettings(&self) -> ::windows::core::Result<AppBroadcastProviderSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastProviderSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Logo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPlugIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastPlugIn {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugIn;{520c1e66-6513-4574-ac54-23b79729615b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastPlugIn as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastPlugIn {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugIn";
}
impl ::core::convert::From<AppBroadcastPlugIn> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPlugIn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugIn> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPlugIn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPlugIn> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPlugIn) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugIn> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPlugIn) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPlugIn {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPlugIn {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugIn {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPlugInManager(::windows::core::IUnknown);
impl AppBroadcastPlugInManager {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsBroadcastProviderAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBroadcastProviderAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PlugInList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlugInList)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AppBroadcastPlugIn>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn DefaultPlugIn(&self) -> ::windows::core::Result<AppBroadcastPlugIn> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultPlugIn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugIn>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetDefaultPlugIn<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastPlugIn>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultPlugIn)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInManager>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<AppBroadcastPlugInManager> {
        Self::IAppBroadcastPlugInManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppBroadcastPlugInManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastPlugInManagerStatics<R, F: FnOnce(&IAppBroadcastPlugInManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppBroadcastPlugInManager, IAppBroadcastPlugInManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppBroadcastPlugInManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastPlugInManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInManager;{e550d979-27a1-49a7-bbf4-d7a9e9d07668})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastPlugInManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastPlugInManager {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInManager";
}
impl ::core::convert::From<AppBroadcastPlugInManager> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPlugInManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugInManager> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPlugInManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPlugInManager> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPlugInManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugInManager> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPlugInManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPlugInManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPlugInManager {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugInManager {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastPlugInState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastPlugInState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPlugInState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPlugInState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPlugInState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPlugInStateChangedEventArgs(::windows::core::IUnknown);
impl AppBroadcastPlugInStateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPlugInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlugInState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPlugInStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastPlugInStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs;{4881d0f2-abc5-4fc6-84b0-89370bb47212})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastPlugInStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastPlugInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPlugInStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastPlugInStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPlugInStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugInStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPlugInStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPlugInStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPlugInStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPlugInStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPlugInStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPlugInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPlugInStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastPlugInStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreview(::windows::core::IUnknown);
impl AppBroadcastPreview {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StopPreview(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopPreview)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PreviewState(&self) -> ::windows::core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPreviewState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviewState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreviewStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastPreview, AppBroadcastPreviewStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviewStateChanged)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePreviewStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePreviewStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PreviewStreamReader(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviewStreamReader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewStreamReader>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreview;{14b60f5a-6e4a-4b80-a14f-67ee77d153e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastPreview {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreview";
}
impl ::core::convert::From<AppBroadcastPreview> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreview> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreview> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreview> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreview {}
unsafe impl ::core::marker::Sync for AppBroadcastPreview {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastPreviewState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastPreviewState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastPreviewState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastPreviewState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreviewStateChangedEventArgs(::windows::core::IUnknown);
impl AppBroadcastPreviewStateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PreviewState(&self) -> ::windows::core::Result<AppBroadcastPreviewState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPreviewState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviewState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPreviewStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs;{5a57f2de-8dea-4e86-90ad-03fc26b9653c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastPreviewStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastPreviewStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastPreviewStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreviewStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreviewStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreviewStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreviewStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreviewStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreviewStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamReader(::windows::core::IUnknown);
impl AppBroadcastPreviewStreamReader {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoStride(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoStride)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoBitmapPixelFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn VideoBitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: super::super::Graphics::Imaging::BitmapAlphaMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoBitmapAlphaMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapAlphaMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TryGetNextVideoFrame(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetNextVideoFrame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewStreamVideoFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastPreviewStreamReader, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoFrameArrived)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVideoFrameArrived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastPreviewStreamReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewStreamReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamReader;{92228d50-db3f-40a8-8cd4-f4e371ddab37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastPreviewStreamReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastPreviewStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamReader";
}
impl ::core::convert::From<AppBroadcastPreviewStreamReader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreviewStreamReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamReader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreviewStreamReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreviewStreamReader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreviewStreamReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamReader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreviewStreamReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreviewStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamReader {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamReader {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoFrame(::windows::core::IUnknown);
impl AppBroadcastPreviewStreamVideoFrame {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoHeader(&self) -> ::windows::core::Result<AppBroadcastPreviewStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoHeader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPreviewStreamVideoHeader>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoBuffer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPreviewStreamVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewStreamVideoFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame;{010fbea1-94fe-4499-b8c0-8d244279fb12})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastPreviewStreamVideoFrame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastPreviewStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoFrame";
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoFrame> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreviewStreamVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoFrame> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreviewStreamVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoFrame> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreviewStreamVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoFrame> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreviewStreamVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreviewStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamVideoFrame {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamVideoFrame {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastPreviewStreamVideoHeader(::windows::core::IUnknown);
impl AppBroadcastPreviewStreamVideoHeader {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbsoluteTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativeTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn FrameId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastPreviewStreamVideoHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastPreviewStreamVideoHeader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader;{8bef6113-da84-4499-a7ab-87118cb4a157})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastPreviewStreamVideoHeader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastPreviewStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastPreviewStreamVideoHeader";
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoHeader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastPreviewStreamVideoHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoHeader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastPreviewStreamVideoHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastPreviewStreamVideoHeader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastPreviewStreamVideoHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastPreviewStreamVideoHeader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastPreviewStreamVideoHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastPreviewStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastPreviewStreamVideoHeader {}
unsafe impl ::core::marker::Sync for AppBroadcastPreviewStreamVideoHeader {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastProviderSettings(::windows::core::IUnknown);
impl AppBroadcastProviderSettings {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetDefaultBroadcastTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultBroadcastTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn DefaultBroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultBroadcastTitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioEncodingBitrate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioEncodingBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioEncodingBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomVideoEncodingBitrate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomVideoEncodingBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomVideoEncodingHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CustomVideoEncodingHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomVideoEncodingHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomVideoEncodingWidth)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CustomVideoEncodingWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomVideoEncodingWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetVideoEncodingBitrateMode(&self, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoEncodingBitrateMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoEncodingBitrateMode(&self) -> ::windows::core::Result<AppBroadcastVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastVideoEncodingBitrateMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoEncodingBitrateMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastVideoEncodingBitrateMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetVideoEncodingResolutionMode(&self, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoEncodingResolutionMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoEncodingResolutionMode(&self) -> ::windows::core::Result<AppBroadcastVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastVideoEncodingResolutionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoEncodingResolutionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastVideoEncodingResolutionMode>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastProviderSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastProviderSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastProviderSettings;{c30bdf62-9948-458f-ad50-aa06ec03da08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastProviderSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastProviderSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastProviderSettings";
}
impl ::core::convert::From<AppBroadcastProviderSettings> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastProviderSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastProviderSettings> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastProviderSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastProviderSettings> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastProviderSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastProviderSettings> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastProviderSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastProviderSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastServices(::windows::core::IUnknown);
impl AppBroadcastServices {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CaptureTargetType(&self) -> ::windows::core::Result<AppBroadcastCaptureTargetType> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCaptureTargetType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureTargetType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCaptureTargetType>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetCaptureTargetType(&self, value: AppBroadcastCaptureTargetType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCaptureTargetType)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn BroadcastTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BroadcastTitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetBroadcastTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBroadcastTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn BroadcastLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BroadcastLanguage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetBroadcastLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBroadcastLanguage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CanCapture(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCapture)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnterBroadcastModeAsync<'a, Param0: ::windows::core::IntoParam<'a, AppBroadcastPlugIn>>(&self, plugin: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnterBroadcastModeAsync)(::core::mem::transmute_copy(this), plugin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ExitBroadcastMode(&self, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ExitBroadcastMode)(::core::mem::transmute_copy(this), reason).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StartBroadcast(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StartBroadcast)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PauseBroadcast(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PauseBroadcast)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ResumeBroadcast(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ResumeBroadcast)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPreview<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, desiredsize: Param0) -> ::windows::core::Result<AppBroadcastPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartPreview)(::core::mem::transmute_copy(this), desiredsize.into_param().abi(), &mut result__).from_abi::<AppBroadcastPreview>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn State(&self) -> ::windows::core::Result<AppBroadcastState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastServices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastServices;{8660b4d6-969b-4e3c-ac3a-8b042ee4ee63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastServices {
    type Vtable = IAppBroadcastServices_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastServices as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastServices {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastServices";
}
impl ::core::convert::From<AppBroadcastServices> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastServices> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastServices> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastServices> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastServices {}
unsafe impl ::core::marker::Sync for AppBroadcastServices {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastSignInResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastSignInResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastSignInResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastSignInState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastSignInState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastSignInState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastSignInState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastSignInState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastSignInStateChangedEventArgs(::windows::core::IUnknown);
impl AppBroadcastSignInStateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignInState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Result(&self) -> ::windows::core::Result<AppBroadcastSignInResult> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInResult>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastSignInStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastSignInStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs;{02b692a4-5919-4a9e-8d5e-c9bb0dd3377a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastSignInStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastSignInStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastSignInStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastSignInStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastSignInStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastSignInStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastSignInStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastSignInStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastSignInStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastSignInStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastSignInStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastSignInStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastState(::windows::core::IUnknown);
impl AppBroadcastState {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsCaptureTargetRunning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCaptureTargetRunning)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ViewerCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewerCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ShouldCaptureMicrophone(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldCaptureMicrophone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShouldCaptureMicrophone)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn RestartMicrophoneCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RestartMicrophoneCapture)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ShouldCaptureCamera(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldCaptureCamera)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetShouldCaptureCamera(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShouldCaptureCamera)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn RestartCameraCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RestartCameraCapture)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EncodedVideoSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncodedVideoSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MicrophoneCaptureState(&self) -> ::windows::core::Result<AppBroadcastMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastMicrophoneCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicrophoneCaptureState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastMicrophoneCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MicrophoneCaptureError(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicrophoneCaptureError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CameraCaptureState(&self) -> ::windows::core::Result<AppBroadcastCameraCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastCameraCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraCaptureState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastCameraCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CameraCaptureError(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraCaptureError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastStreamState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PlugInState(&self) -> ::windows::core::Result<AppBroadcastPlugInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastPlugInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlugInState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastPlugInState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OAuthRequestUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OAuthRequestUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OAuthCallbackUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OAuthCallbackUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn AuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationResult)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn SetAuthenticationResult<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Authentication::Web::WebAuthenticationResult>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAuthenticationResult)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetSignInState(&self, value: AppBroadcastSignInState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSignInState)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SignInState(&self) -> ::windows::core::Result<AppBroadcastSignInState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastSignInState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignInState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastSignInState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TerminationReason(&self) -> ::windows::core::Result<AppBroadcastTerminationReason> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastTerminationReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TerminationReason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastTerminationReason>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TerminationReasonPlugInSpecific(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TerminationReasonPlugInSpecific)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ViewerCountChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastViewerCountChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewerCountChanged)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveViewerCountChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveViewerCountChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MicrophoneCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastMicrophoneCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicrophoneCaptureStateChanged)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMicrophoneCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMicrophoneCaptureStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastCameraCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraCaptureStateChanged)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCameraCaptureStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlugInStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastPlugInStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PlugInStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlugInStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePlugInStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, AppBroadcastStreamStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStreamStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTargetClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastState, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureTargetClosed)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCaptureTargetClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCaptureTargetClosed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastState;{ee08056d-8099-4ddd-922e-c56dac58abfb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastState {
    type Vtable = IAppBroadcastState_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastState as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastState {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastState";
}
impl ::core::convert::From<AppBroadcastState> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastState> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastState> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastState> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastState {}
unsafe impl ::core::marker::Sync for AppBroadcastState {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamAudioFrame(::windows::core::IUnknown);
impl AppBroadcastStreamAudioFrame {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioHeader(&self) -> ::windows::core::Result<AppBroadcastStreamAudioHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioHeader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamAudioHeader>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioBuffer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamAudioFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamAudioFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioFrame;{efab4ac8-21ba-453f-8bb7-5e938a2e9a74})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastStreamAudioFrame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastStreamAudioFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioFrame";
}
impl ::core::convert::From<AppBroadcastStreamAudioFrame> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamAudioFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioFrame> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamAudioFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamAudioFrame> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamAudioFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioFrame> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamAudioFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamAudioFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamAudioHeader(::windows::core::IUnknown);
impl AppBroadcastStreamAudioHeader {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbsoluteTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativeTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn HasDiscontinuity(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasDiscontinuity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn FrameId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamAudioHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamAudioHeader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamAudioHeader;{bf21a570-6b78-4216-9f07-5aff5256f1b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastStreamAudioHeader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastStreamAudioHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamAudioHeader";
}
impl ::core::convert::From<AppBroadcastStreamAudioHeader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamAudioHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioHeader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamAudioHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamAudioHeader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamAudioHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamAudioHeader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamAudioHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamAudioHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamReader(::windows::core::IUnknown);
impl AppBroadcastStreamReader {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioChannels(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioChannels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioSampleRate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioSampleRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AudioAacSequence(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioAacSequence)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TryGetNextAudioFrame(&self) -> ::windows::core::Result<AppBroadcastStreamAudioFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetNextAudioFrame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamAudioFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TryGetNextVideoFrame(&self) -> ::windows::core::Result<AppBroadcastStreamVideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryGetNextVideoFrame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamVideoFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AudioFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioFrameArrived)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAudioFrameArrived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastStreamReader, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoFrameArrived)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoFrameArrived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVideoFrameArrived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamReader;{b338bcf9-3364-4460-b5f1-3cc2796a8aa2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastStreamReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastStreamReader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamReader";
}
impl ::core::convert::From<AppBroadcastStreamReader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamReader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamReader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamReader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastStreamState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastStreamState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastStreamState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastStreamState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamStateChangedEventArgs(::windows::core::IUnknown);
impl AppBroadcastStreamStateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StreamState(&self) -> ::windows::core::Result<AppBroadcastStreamState> {
        let this = self;
        unsafe {
            let mut result__: AppBroadcastStreamState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs;{5108a733-d008-4a89-93be-58aed961374e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastStreamStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastStreamStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamStateChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastStreamStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamVideoFrame(::windows::core::IUnknown);
impl AppBroadcastStreamVideoFrame {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoHeader(&self) -> ::windows::core::Result<AppBroadcastStreamVideoHeader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoHeader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastStreamVideoHeader>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn VideoBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoBuffer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamVideoFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamVideoFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoFrame;{0f97cf2b-c9e4-4e88-8194-d814cbd585d8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastStreamVideoFrame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastStreamVideoFrame {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoFrame";
}
impl ::core::convert::From<AppBroadcastStreamVideoFrame> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoFrame> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamVideoFrame> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamVideoFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoFrame> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamVideoFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamVideoFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastStreamVideoHeader(::windows::core::IUnknown);
impl AppBroadcastStreamVideoHeader {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AbsoluteTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AbsoluteTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativeTimestamp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsKeyFrame(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsKeyFrame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn HasDiscontinuity(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasDiscontinuity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn FrameId(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastStreamVideoHeader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastStreamVideoHeader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastStreamVideoHeader;{0b9ebece-7e32-432d-8ca2-36bf10b9f462})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastStreamVideoHeader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastStreamVideoHeader {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastStreamVideoHeader";
}
impl ::core::convert::From<AppBroadcastStreamVideoHeader> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastStreamVideoHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoHeader> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastStreamVideoHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastStreamVideoHeader> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastStreamVideoHeader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastStreamVideoHeader> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastStreamVideoHeader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastStreamVideoHeader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastTerminationReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastTerminationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastTerminationReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastTerminationReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastTerminationReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastTriggerDetails(::windows::core::IUnknown);
impl AppBroadcastTriggerDetails {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn BackgroundService(&self) -> ::windows::core::Result<AppBroadcastBackgroundService> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundService)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastBackgroundService>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastTriggerDetails;{deebab35-ec5e-4d8f-b1c0-5da6e8c75638})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastTriggerDetails {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastTriggerDetails";
}
impl ::core::convert::From<AppBroadcastTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastVideoEncodingBitrateMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastVideoEncodingBitrateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastVideoEncodingBitrateMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastVideoEncodingBitrateMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingBitrateMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppBroadcastVideoEncodingResolutionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppBroadcastVideoEncodingResolutionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastVideoEncodingResolutionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastVideoEncodingResolutionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppBroadcastVideoEncodingResolutionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppBroadcastViewerCountChangedEventArgs(::windows::core::IUnknown);
impl AppBroadcastViewerCountChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ViewerCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewerCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastViewerCountChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppBroadcastViewerCountChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs;{e6e11825-5401-4ade-8bd2-c14ecee6807d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastViewerCountChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastViewerCountChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppBroadcastViewerCountChangedEventArgs";
}
impl ::core::convert::From<AppBroadcastViewerCountChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastViewerCountChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastViewerCountChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastViewerCountChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastViewerCountChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastViewerCountChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastViewerCountChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastViewerCountChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastViewerCountChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastViewerCountChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppBroadcastViewerCountChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCapture(::windows::core::IUnknown);
impl AppCapture {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsCapturingAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCapturingAudio)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsCapturingVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCapturingVideo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CapturingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCapture, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CapturingChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCapturingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCapturingChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<AppCapture> {
        Self::IAppCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCapture>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAllowedAsync(allowed: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppCaptureStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetAllowedAsync)(::core::mem::transmute_copy(this), allowed, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppCaptureStatics<R, F: FnOnce(&IAppCaptureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCapture, IAppCaptureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAppCaptureStatics2<R, F: FnOnce(&IAppCaptureStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCapture, IAppCaptureStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCapture;{9749d453-a29a-45ed-8f29-22d09942cff7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCapture {
    type Vtable = IAppCapture_Vtbl;
    const IID: ::windows::core::GUID = <IAppCapture as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCapture {
    const NAME: &'static str = "Windows.Media.Capture.AppCapture";
}
impl ::core::convert::From<AppCapture> for ::windows::core::IUnknown {
    fn from(value: AppCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapture> for ::windows::core::IUnknown {
    fn from(value: &AppCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCapture> for ::windows::core::IInspectable {
    fn from(value: AppCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapture> for ::windows::core::IInspectable {
    fn from(value: &AppCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureAlternateShortcutKeys(::windows::core::IUnknown);
impl AppCaptureAlternateShortcutKeys {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleGameBarKey)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleGameBarKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleGameBarKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleGameBarKeyModifiers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleGameBarKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleGameBarKeyModifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSaveHistoricalVideoKey)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveHistoricalVideoKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetSaveHistoricalVideoKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSaveHistoricalVideoKeyModifiers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SaveHistoricalVideoKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveHistoricalVideoKeyModifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleRecordingKey)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleRecordingKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleRecordingKeyModifiers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleRecordingKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleRecordingKeyModifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTakeScreenshotKey)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TakeScreenshotKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetTakeScreenshotKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTakeScreenshotKeyModifiers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn TakeScreenshotKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TakeScreenshotKeyModifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleRecordingIndicatorKey)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleRecordingIndicatorKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleRecordingIndicatorKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleRecordingIndicatorKeyModifiers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleRecordingIndicatorKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = self;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleRecordingIndicatorKeyModifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleMicrophoneCaptureKey)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleMicrophoneCaptureKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleMicrophoneCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleMicrophoneCaptureKeyModifiers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleMicrophoneCaptureKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys2>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleMicrophoneCaptureKeyModifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleCameraCaptureKey)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleCameraCaptureKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleCameraCaptureKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleCameraCaptureKeyModifiers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleCameraCaptureKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleCameraCaptureKeyModifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKey(&self, value: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleBroadcastKey)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKey = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleBroadcastKey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetToggleBroadcastKeyModifiers(&self, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetToggleBroadcastKeyModifiers)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ToggleBroadcastKeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers> {
        let this = &::windows::core::Interface::cast::<IAppCaptureAlternateShortcutKeys3>(self)?;
        unsafe {
            let mut result__: super::super::System::VirtualKeyModifiers = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToggleBroadcastKeyModifiers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::VirtualKeyModifiers>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureAlternateShortcutKeys {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureAlternateShortcutKeys {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureAlternateShortcutKeys;{19e8e0ef-236c-40f9-b38f-9b7dd65d1ccc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureAlternateShortcutKeys as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureAlternateShortcutKeys {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureAlternateShortcutKeys";
}
impl ::core::convert::From<AppCaptureAlternateShortcutKeys> for ::windows::core::IUnknown {
    fn from(value: AppCaptureAlternateShortcutKeys) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureAlternateShortcutKeys> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureAlternateShortcutKeys) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureAlternateShortcutKeys> for ::windows::core::IInspectable {
    fn from(value: AppCaptureAlternateShortcutKeys) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureAlternateShortcutKeys> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureAlternateShortcutKeys) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureAlternateShortcutKeys {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureDurationGeneratedEventArgs(::windows::core::IUnknown);
impl AppCaptureDurationGeneratedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureDurationGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureDurationGeneratedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs;{c1f5563b-ffa1-44c9-975f-27fbeb553b35})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureDurationGeneratedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureDurationGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureDurationGeneratedEventArgs";
}
impl ::core::convert::From<AppCaptureDurationGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCaptureDurationGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureDurationGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureDurationGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureDurationGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCaptureDurationGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureDurationGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureDurationGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureDurationGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureDurationGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureDurationGeneratedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureFileGeneratedEventArgs(::windows::core::IUnknown);
impl AppCaptureFileGeneratedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureFileGeneratedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureFileGeneratedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureFileGeneratedEventArgs;{4189fbf4-465e-45bf-907f-165b3fb23758})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureFileGeneratedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureFileGeneratedEventArgs";
}
impl ::core::convert::From<AppCaptureFileGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCaptureFileGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureFileGeneratedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureFileGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureFileGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCaptureFileGeneratedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureFileGeneratedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureFileGeneratedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureFileGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureFileGeneratedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppCaptureHistoricalBufferLengthUnit {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureHistoricalBufferLengthUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureHistoricalBufferLengthUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureHistoricalBufferLengthUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureHistoricalBufferLengthUnit;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
pub struct AppCaptureManager {}
impl AppCaptureManager {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetCurrentSettings() -> ::windows::core::Result<AppCaptureSettings> {
        Self::IAppCaptureManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureSettings>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ApplySettings<'a, Param0: ::windows::core::IntoParam<'a, AppCaptureSettings>>(appcapturesettings: Param0) -> ::windows::core::Result<()> {
        Self::IAppCaptureManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ApplySettings)(::core::mem::transmute_copy(this), appcapturesettings.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IAppCaptureManagerStatics<R, F: FnOnce(&IAppCaptureManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCaptureManager, IAppCaptureManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppCaptureManager {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureManager";
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppCaptureMetadataPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureMetadataPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMetadataPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureMetadataPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMetadataPriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureMetadataWriter(::windows::core::IUnknown);
impl AppCaptureMetadataWriter {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCaptureMetadataWriter, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AddStringEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddStringEvent)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), priority).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AddInt32Event<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInt32Event)(::core::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AddDoubleEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDoubleEvent)(::core::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StartStringState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: Param1, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StartStringState)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), priority).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StartInt32State<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StartInt32State)(::core::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StartDoubleState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StartDoubleState)(::core::mem::transmute_copy(this), name.into_param().abi(), value, priority).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StopState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopState)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StopAllStates(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopAllStates)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn RemainingStorageBytesAvailable(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemainingStorageBytesAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MetadataPurged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureMetadataWriter, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MetadataPurged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMetadataPurged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMetadataPurged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AppCaptureMetadataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureMetadataWriter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMetadataWriter;{e0ce4877-9aaf-46b4-ad31-6a60b441c780})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureMetadataWriter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureMetadataWriter {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMetadataWriter";
}
impl ::core::convert::From<AppCaptureMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: AppCaptureMetadataWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureMetadataWriter> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureMetadataWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureMetadataWriter> for ::windows::core::IInspectable {
    fn from(value: AppCaptureMetadataWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureMetadataWriter> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureMetadataWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<AppCaptureMetadataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: AppCaptureMetadataWriter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&AppCaptureMetadataWriter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppCaptureMetadataWriter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &AppCaptureMetadataWriter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppCaptureMetadataWriter {}
unsafe impl ::core::marker::Sync for AppCaptureMetadataWriter {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppCaptureMicrophoneCaptureState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureMicrophoneCaptureState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureMicrophoneCaptureState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureMicrophoneCaptureState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureMicrophoneCaptureState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureMicrophoneCaptureStateChangedEventArgs(::windows::core::IUnknown);
impl AppCaptureMicrophoneCaptureStateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn State(&self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureMicrophoneCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureMicrophoneCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs;{324d249e-45bc-4c35-bc35-e469fc7a69e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureMicrophoneCaptureStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureMicrophoneCaptureStateChangedEventArgs";
}
impl ::core::convert::From<AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureMicrophoneCaptureStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureMicrophoneCaptureStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureMicrophoneCaptureStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureMicrophoneCaptureStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureRecordOperation(::windows::core::IUnknown);
impl AppCaptureRecordOperation {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StopRecording(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StopRecording)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn State(&self) -> ::windows::core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureRecordingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureRecordingState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsFileTruncated(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFileTruncated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureRecordingStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StateChanged)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DurationGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureDurationGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DurationGenerated)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDurationGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDurationGenerated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureRecordOperation, AppCaptureFileGeneratedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileGenerated)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileGenerated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFileGenerated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppCaptureRecordOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureRecordOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordOperation;{c66020a9-1538-495c-9bbb-2ba870ec5861})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureRecordOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureRecordOperation {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordOperation";
}
impl ::core::convert::From<AppCaptureRecordOperation> for ::windows::core::IUnknown {
    fn from(value: AppCaptureRecordOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureRecordOperation> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureRecordOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureRecordOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureRecordOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureRecordOperation> for ::windows::core::IInspectable {
    fn from(value: AppCaptureRecordOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureRecordOperation> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureRecordOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureRecordOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureRecordOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureRecordOperation {}
unsafe impl ::core::marker::Sync for AppCaptureRecordOperation {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppCaptureRecordingState {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureRecordingState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureRecordingState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureRecordingState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureRecordingState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureRecordingStateChangedEventArgs(::windows::core::IUnknown);
impl AppCaptureRecordingStateChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn State(&self) -> ::windows::core::Result<AppCaptureRecordingState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureRecordingState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureRecordingState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureRecordingStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureRecordingStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs;{24fc8712-e305-490d-b415-6b1c9049736b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureRecordingStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureRecordingStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureRecordingStateChangedEventArgs";
}
impl ::core::convert::From<AppCaptureRecordingStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCaptureRecordingStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureRecordingStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureRecordingStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureRecordingStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCaptureRecordingStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureRecordingStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureRecordingStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureRecordingStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureRecordingStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCaptureRecordingStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureServices(::windows::core::IUnknown);
impl AppCaptureServices {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Record(&self) -> ::windows::core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Record)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureRecordOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecordTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, starttime: Param0, duration: Param1) -> ::windows::core::Result<AppCaptureRecordOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordTimeSpan)(::core::mem::transmute_copy(this), starttime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<AppCaptureRecordOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CanCapture(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCapture)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn State(&self) -> ::windows::core::Result<AppCaptureState> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureState>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureServices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureServices;{44fec0b5-34f5-4f18-ae8c-b9123abbfc0d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureServices {
    type Vtable = IAppCaptureServices_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureServices as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureServices {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureServices";
}
impl ::core::convert::From<AppCaptureServices> for ::windows::core::IUnknown {
    fn from(value: AppCaptureServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureServices> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureServices> for ::windows::core::IInspectable {
    fn from(value: AppCaptureServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureServices> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureServices {}
unsafe impl ::core::marker::Sync for AppCaptureServices {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureSettings(::windows::core::IUnknown);
impl AppCaptureSettings {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SetAppCaptureDestinationFolder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFolder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAppCaptureDestinationFolder)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn AppCaptureDestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppCaptureDestinationFolder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetAudioEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioEncodingBitrate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioEncodingBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioEncodingBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsAudioCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAudioCaptureEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsAudioCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAudioCaptureEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetCustomVideoEncodingBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomVideoEncodingBitrate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CustomVideoEncodingBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomVideoEncodingBitrate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetCustomVideoEncodingHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomVideoEncodingHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CustomVideoEncodingHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomVideoEncodingHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetCustomVideoEncodingWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomVideoEncodingWidth)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CustomVideoEncodingWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomVideoEncodingWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetHistoricalBufferLength(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHistoricalBufferLength)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn HistoricalBufferLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HistoricalBufferLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetHistoricalBufferLengthUnit(&self, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHistoricalBufferLengthUnit)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn HistoricalBufferLengthUnit(&self) -> ::windows::core::Result<AppCaptureHistoricalBufferLengthUnit> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureHistoricalBufferLengthUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HistoricalBufferLengthUnit)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureHistoricalBufferLengthUnit>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsHistoricalCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsHistoricalCaptureEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHistoricalCaptureEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsHistoricalCaptureOnBatteryAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsHistoricalCaptureOnBatteryAllowed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsHistoricalCaptureOnBatteryAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHistoricalCaptureOnBatteryAllowed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsHistoricalCaptureOnWirelessDisplayAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsHistoricalCaptureOnWirelessDisplayAllowed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsHistoricalCaptureOnWirelessDisplayAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHistoricalCaptureOnWirelessDisplayAllowed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaximumRecordLength<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaximumRecordLength)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaximumRecordLength(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaximumRecordLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SetScreenshotDestinationFolder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFolder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScreenshotDestinationFolder)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn ScreenshotDestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScreenshotDestinationFolder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetVideoEncodingBitrateMode(&self, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoEncodingBitrateMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoEncodingBitrateMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingBitrateMode> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureVideoEncodingBitrateMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoEncodingBitrateMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureVideoEncodingBitrateMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetVideoEncodingResolutionMode(&self, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoEncodingResolutionMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoEncodingResolutionMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingResolutionMode> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureVideoEncodingResolutionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoEncodingResolutionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureVideoEncodingResolutionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsAppCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAppCaptureEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsAppCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAppCaptureEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsCpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCpuConstrained)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsDisabledByPolicy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisabledByPolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsMemoryConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMemoryConstrained)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn HasHardwareEncoder(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasHardwareEncoder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsGpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGpuConstrained)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AlternateShortcutKeys(&self) -> ::windows::core::Result<AppCaptureAlternateShortcutKeys> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlternateShortcutKeys)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureAlternateShortcutKeys>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsMicrophoneCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsMicrophoneCaptureEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsMicrophoneCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMicrophoneCaptureEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsMicrophoneCaptureEnabledByDefault(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsMicrophoneCaptureEnabledByDefault)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsMicrophoneCaptureEnabledByDefault(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsMicrophoneCaptureEnabledByDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetSystemAudioGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSystemAudioGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SystemAudioGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemAudioGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetMicrophoneGain(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMicrophoneGain)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MicrophoneGain(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicrophoneGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetVideoEncodingFrameRateMode(&self, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoEncodingFrameRateMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoEncodingFrameRateMode(&self) -> ::windows::core::Result<AppCaptureVideoEncodingFrameRateMode> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings4>(self)?;
        unsafe {
            let mut result__: AppCaptureVideoEncodingFrameRateMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoEncodingFrameRateMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureVideoEncodingFrameRateMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsEchoCancellationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEchoCancellationEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsEchoCancellationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEchoCancellationEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetIsCursorImageCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCursorImageCaptureEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsCursorImageCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IAppCaptureSettings5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCursorImageCaptureEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureSettings;{14683a86-8807-48d3-883a-970ee4532a39})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureSettings {
    type Vtable = IAppCaptureSettings_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureSettings";
}
impl ::core::convert::From<AppCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: AppCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: AppCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct AppCaptureState(::windows::core::IUnknown);
impl AppCaptureState {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsTargetRunning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTargetRunning)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsHistoricalCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHistoricalCaptureEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ShouldCaptureMicrophone(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShouldCaptureMicrophone)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetShouldCaptureMicrophone(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShouldCaptureMicrophone)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn RestartMicrophoneCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RestartMicrophoneCapture)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MicrophoneCaptureState(&self) -> ::windows::core::Result<AppCaptureMicrophoneCaptureState> {
        let this = self;
        unsafe {
            let mut result__: AppCaptureMicrophoneCaptureState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicrophoneCaptureState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureMicrophoneCaptureState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MicrophoneCaptureError(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicrophoneCaptureError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MicrophoneCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureState, AppCaptureMicrophoneCaptureStateChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicrophoneCaptureStateChanged)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMicrophoneCaptureStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMicrophoneCaptureStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTargetClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppCaptureState, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureTargetClosed)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCaptureTargetClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCaptureTargetClosed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppCaptureState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for AppCaptureState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.AppCaptureState;{73134372-d4eb-44ce-9538-465f506ac4ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppCaptureState {
    type Vtable = IAppCaptureState_Vtbl;
    const IID: ::windows::core::GUID = <IAppCaptureState as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCaptureState {
    const NAME: &'static str = "Windows.Media.Capture.AppCaptureState";
}
impl ::core::convert::From<AppCaptureState> for ::windows::core::IUnknown {
    fn from(value: AppCaptureState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureState> for ::windows::core::IUnknown {
    fn from(value: &AppCaptureState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCaptureState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCaptureState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCaptureState> for ::windows::core::IInspectable {
    fn from(value: AppCaptureState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCaptureState> for ::windows::core::IInspectable {
    fn from(value: &AppCaptureState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCaptureState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCaptureState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCaptureState {}
unsafe impl ::core::marker::Sync for AppCaptureState {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppCaptureVideoEncodingBitrateMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureVideoEncodingBitrateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingBitrateMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureVideoEncodingBitrateMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingBitrateMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppCaptureVideoEncodingFrameRateMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureVideoEncodingFrameRateMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingFrameRateMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureVideoEncodingFrameRateMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingFrameRateMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for AppCaptureVideoEncodingResolutionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCaptureVideoEncodingResolutionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCaptureVideoEncodingResolutionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCaptureVideoEncodingResolutionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.AppCaptureVideoEncodingResolutionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUI(::windows::core::IUnknown);
impl CameraCaptureUI {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CameraCaptureUI, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PhotoSettings(&self) -> ::windows::core::Result<CameraCaptureUIPhotoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhotoSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIPhotoCaptureSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoSettings(&self) -> ::windows::core::Result<CameraCaptureUIVideoCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIVideoCaptureSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CaptureFileAsync(&self, mode: CameraCaptureUIMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureFileAsync)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
}
impl ::core::clone::Clone for CameraCaptureUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CameraCaptureUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUI;{48587540-6f93-4bb4-b8f3-e89e48948c91})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CameraCaptureUI {
    type Vtable = ICameraCaptureUI_Vtbl;
    const IID: ::windows::core::GUID = <ICameraCaptureUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CameraCaptureUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUI";
}
impl ::core::convert::From<CameraCaptureUI> for ::windows::core::IUnknown {
    fn from(value: CameraCaptureUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUI> for ::windows::core::IUnknown {
    fn from(value: &CameraCaptureUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraCaptureUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraCaptureUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraCaptureUI> for ::windows::core::IInspectable {
    fn from(value: CameraCaptureUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUI> for ::windows::core::IInspectable {
    fn from(value: &CameraCaptureUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraCaptureUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraCaptureUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CameraCaptureUIMaxPhotoResolution {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIMaxPhotoResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMaxPhotoResolution").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIMaxPhotoResolution {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxPhotoResolution;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CameraCaptureUIMaxVideoResolution {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIMaxVideoResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMaxVideoResolution").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIMaxVideoResolution {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMaxVideoResolution;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CameraCaptureUIMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUIPhotoCaptureSettings(::windows::core::IUnknown);
impl CameraCaptureUIPhotoCaptureSettings {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Format(&self) -> ::windows::core::Result<CameraCaptureUIPhotoFormat> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIPhotoFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Format)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIPhotoFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetFormat(&self, value: CameraCaptureUIPhotoFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFormat)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MaxResolution(&self) -> ::windows::core::Result<CameraCaptureUIMaxPhotoResolution> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIMaxPhotoResolution = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxResolution)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIMaxPhotoResolution>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxResolution)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CroppedSizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CroppedSizeInPixels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCroppedSizeInPixels<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCroppedSizeInPixels)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CroppedAspectRatio(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CroppedAspectRatio)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCroppedAspectRatio<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCroppedAspectRatio)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AllowCropping(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowCropping)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetAllowCropping(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowCropping)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for CameraCaptureUIPhotoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIPhotoCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings;{b9f5be97-3472-46a8-8a9e-04ce42ccc97d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_Vtbl;
    const IID: ::windows::core::GUID = <ICameraCaptureUIPhotoCaptureSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CameraCaptureUIPhotoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIPhotoCaptureSettings";
}
impl ::core::convert::From<CameraCaptureUIPhotoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: CameraCaptureUIPhotoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUIPhotoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &CameraCaptureUIPhotoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraCaptureUIPhotoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: CameraCaptureUIPhotoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUIPhotoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &CameraCaptureUIPhotoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraCaptureUIPhotoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CameraCaptureUIPhotoCaptureSettings {}
unsafe impl ::core::marker::Sync for CameraCaptureUIPhotoCaptureSettings {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CameraCaptureUIPhotoFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIPhotoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIPhotoFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIPhotoFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIPhotoFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CameraCaptureUIVideoCaptureSettings(::windows::core::IUnknown);
impl CameraCaptureUIVideoCaptureSettings {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Format(&self) -> ::windows::core::Result<CameraCaptureUIVideoFormat> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIVideoFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Format)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIVideoFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetFormat(&self, value: CameraCaptureUIVideoFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFormat)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MaxResolution(&self) -> ::windows::core::Result<CameraCaptureUIMaxVideoResolution> {
        let this = self;
        unsafe {
            let mut result__: CameraCaptureUIMaxVideoResolution = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxResolution)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CameraCaptureUIMaxVideoResolution>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetMaxResolution(&self, value: CameraCaptureUIMaxVideoResolution) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxResolution)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MaxDurationInSeconds(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxDurationInSeconds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetMaxDurationInSeconds(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxDurationInSeconds)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AllowTrimming(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowTrimming)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetAllowTrimming(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowTrimming)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for CameraCaptureUIVideoCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIVideoCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings;{64e92d1f-a28d-425a-b84f-e568335ff24e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_Vtbl;
    const IID: ::windows::core::GUID = <ICameraCaptureUIVideoCaptureSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CameraCaptureUIVideoCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.CameraCaptureUIVideoCaptureSettings";
}
impl ::core::convert::From<CameraCaptureUIVideoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: CameraCaptureUIVideoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUIVideoCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &CameraCaptureUIVideoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CameraCaptureUIVideoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: CameraCaptureUIVideoCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraCaptureUIVideoCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &CameraCaptureUIVideoCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraCaptureUIVideoCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CameraCaptureUIVideoCaptureSettings {}
unsafe impl ::core::marker::Sync for CameraCaptureUIVideoCaptureSettings {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CameraCaptureUIVideoFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for CameraCaptureUIVideoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraCaptureUIVideoFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraCaptureUIVideoFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.CameraCaptureUIVideoFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
pub struct CameraOptionsUI {}
impl CameraOptionsUI {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Show<'a, Param0: ::windows::core::IntoParam<'a, MediaCapture>>(mediacapture: Param0) -> ::windows::core::Result<()> {
        Self::ICameraOptionsUIStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Show)(::core::mem::transmute_copy(this), mediacapture.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ICameraOptionsUIStatics<R, F: FnOnce(&ICameraOptionsUIStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CameraOptionsUI, ICameraOptionsUIStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CameraOptionsUI {
    const NAME: &'static str = "Windows.Media.Capture.CameraOptionsUI";
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CapturedFrame(::windows::core::IUnknown);
impl CapturedFrame {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ControlValues(&self) -> ::windows::core::Result<CapturedFrameControlValues> {
        let this = &::windows::core::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ControlValues)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrameControlValues>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn BitmapProperties(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPropertySet> {
        let this = &::windows::core::Interface::cast::<ICapturedFrame2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitmapProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::BitmapPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SoftwareBitmap)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::Imaging::SoftwareBitmap>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadAsync)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WriteAsync)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlushAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSize)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInputStreamAt)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetOutputStreamAt)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Seek)(::core::mem::transmute_copy(this), position).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CloneStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanRead)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanWrite)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CapturedFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CapturedFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrame;{1dd2de1f-571b-44d8-8e80-a08a1578766e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CapturedFrame {
    type Vtable = ICapturedFrame_Vtbl;
    const IID: ::windows::core::GUID = <ICapturedFrame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CapturedFrame {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrame";
}
impl ::core::convert::From<CapturedFrame> for ::windows::core::IUnknown {
    fn from(value: CapturedFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedFrame> for ::windows::core::IUnknown {
    fn from(value: &CapturedFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CapturedFrame> for ::windows::core::IInspectable {
    fn from(value: CapturedFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedFrame> for ::windows::core::IInspectable {
    fn from(value: &CapturedFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IContentTypeProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<CapturedFrame> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: CapturedFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&CapturedFrame> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: &CapturedFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for &CapturedFrame {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CapturedFrame {}
unsafe impl ::core::marker::Sync for CapturedFrame {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CapturedFrameControlValues(::windows::core::IUnknown);
impl CapturedFrameControlValues {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Exposure(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Exposure)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExposureCompensation(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExposureCompensation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsoSpeed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsoSpeed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Focus(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Focus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn SceneMode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SceneMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Devices::CaptureSceneMode>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Flashed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Flashed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlashPowerPercent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FlashPowerPercent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WhiteBalance(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WhiteBalance)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ZoomFactor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ZoomFactor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn FocusState(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Devices::MediaCaptureFocusState>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsoDigitalGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsoDigitalGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsoAnalogGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsoAnalogGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SensorFrameRate(&self) -> ::windows::core::Result<super::MediaProperties::MediaRatio> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SensorFrameRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WhiteBalanceGain(&self) -> ::windows::core::Result<super::super::Foundation::IReference<WhiteBalanceGain>> {
        let this = &::windows::core::Interface::cast::<ICapturedFrameControlValues2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WhiteBalanceGain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<WhiteBalanceGain>>(result__)
        }
    }
}
impl ::core::clone::Clone for CapturedFrameControlValues {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CapturedFrameControlValues {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedFrameControlValues;{90c65b7f-4e0d-4ca4-882d-7a144fed0a90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_Vtbl;
    const IID: ::windows::core::GUID = <ICapturedFrameControlValues as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CapturedFrameControlValues {
    const NAME: &'static str = "Windows.Media.Capture.CapturedFrameControlValues";
}
impl ::core::convert::From<CapturedFrameControlValues> for ::windows::core::IUnknown {
    fn from(value: CapturedFrameControlValues) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedFrameControlValues> for ::windows::core::IUnknown {
    fn from(value: &CapturedFrameControlValues) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CapturedFrameControlValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CapturedFrameControlValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CapturedFrameControlValues> for ::windows::core::IInspectable {
    fn from(value: CapturedFrameControlValues) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedFrameControlValues> for ::windows::core::IInspectable {
    fn from(value: &CapturedFrameControlValues) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CapturedFrameControlValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CapturedFrameControlValues {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CapturedFrameControlValues {}
unsafe impl ::core::marker::Sync for CapturedFrameControlValues {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct CapturedPhoto(::windows::core::IUnknown);
impl CapturedPhoto {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Frame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Thumbnail(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
}
impl ::core::clone::Clone for CapturedPhoto {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for CapturedPhoto {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.CapturedPhoto;{b0ce7e5a-cfcc-4d6c-8ad1-0869208aca16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CapturedPhoto {
    type Vtable = ICapturedPhoto_Vtbl;
    const IID: ::windows::core::GUID = <ICapturedPhoto as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CapturedPhoto {
    const NAME: &'static str = "Windows.Media.Capture.CapturedPhoto";
}
impl ::core::convert::From<CapturedPhoto> for ::windows::core::IUnknown {
    fn from(value: CapturedPhoto) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedPhoto> for ::windows::core::IUnknown {
    fn from(value: &CapturedPhoto) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CapturedPhoto> for ::windows::core::IInspectable {
    fn from(value: CapturedPhoto) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CapturedPhoto> for ::windows::core::IInspectable {
    fn from(value: &CapturedPhoto) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CapturedPhoto {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CapturedPhoto {}
unsafe impl ::core::marker::Sync for CapturedPhoto {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for ForegroundActivationArgument {
    type Abi = Self;
}
impl ::core::fmt::Debug for ForegroundActivationArgument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ForegroundActivationArgument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ForegroundActivationArgument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.ForegroundActivationArgument;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for GameBarCommand {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameBarCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameBarCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommand;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for GameBarCommandOrigin {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameBarCommandOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarCommandOrigin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameBarCommandOrigin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarCommandOrigin;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServices(::windows::core::IUnknown);
impl GameBarServices {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TargetCapturePolicy(&self) -> ::windows::core::Result<GameBarTargetCapturePolicy> {
        let this = self;
        unsafe {
            let mut result__: GameBarTargetCapturePolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetCapturePolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarTargetCapturePolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn EnableCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EnableCapture)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn DisableCapture(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DisableCapture)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TargetInfo(&self) -> ::windows::core::Result<GameBarServicesTargetInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServicesTargetInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AppBroadcastServices(&self) -> ::windows::core::Result<AppBroadcastServices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppBroadcastServices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastServices>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AppCaptureServices(&self) -> ::windows::core::Result<AppCaptureServices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppCaptureServices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCaptureServices>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CommandReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GameBarServices, GameBarServicesCommandEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CommandReceived)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCommandReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCommandReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for GameBarServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for GameBarServices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServices;{2dbead57-50a6-499e-8c6c-d330a7311796})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GameBarServices {
    type Vtable = IGameBarServices_Vtbl;
    const IID: ::windows::core::GUID = <IGameBarServices as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameBarServices {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServices";
}
impl ::core::convert::From<GameBarServices> for ::windows::core::IUnknown {
    fn from(value: GameBarServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServices> for ::windows::core::IUnknown {
    fn from(value: &GameBarServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServices> for ::windows::core::IInspectable {
    fn from(value: GameBarServices) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServices> for ::windows::core::IInspectable {
    fn from(value: &GameBarServices) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServices {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServices {}
unsafe impl ::core::marker::Sync for GameBarServices {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServicesCommandEventArgs(::windows::core::IUnknown);
impl GameBarServicesCommandEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Command(&self) -> ::windows::core::Result<GameBarCommand> {
        let this = self;
        unsafe {
            let mut result__: GameBarCommand = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Command)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarCommand>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Origin(&self) -> ::windows::core::Result<GameBarCommandOrigin> {
        let this = self;
        unsafe {
            let mut result__: GameBarCommandOrigin = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Origin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarCommandOrigin>(result__)
        }
    }
}
impl ::core::clone::Clone for GameBarServicesCommandEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for GameBarServicesCommandEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesCommandEventArgs;{a74226b2-f176-4fcf-8fbb-cf698b2eb8e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IGameBarServicesCommandEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameBarServicesCommandEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesCommandEventArgs";
}
impl ::core::convert::From<GameBarServicesCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: GameBarServicesCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesCommandEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GameBarServicesCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServicesCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: GameBarServicesCommandEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesCommandEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GameBarServicesCommandEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServicesCommandEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServicesCommandEventArgs {}
unsafe impl ::core::marker::Sync for GameBarServicesCommandEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for GameBarServicesDisplayMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameBarServicesDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarServicesDisplayMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameBarServicesDisplayMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarServicesDisplayMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServicesManager(::windows::core::IUnknown);
impl GameBarServicesManager {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GameBarServicesCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GameBarServicesManager, GameBarServicesManagerGameBarServicesCreatedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GameBarServicesCreated)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGameBarServicesCreated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGameBarServicesCreated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<GameBarServicesManager> {
        Self::IGameBarServicesManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServicesManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGameBarServicesManagerStatics<R, F: FnOnce(&IGameBarServicesManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GameBarServicesManager, IGameBarServicesManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for GameBarServicesManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for GameBarServicesManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManager;{3a4b9cfa-7f8b-4c60-9dbb-0bcd262dffc6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GameBarServicesManager {
    type Vtable = IGameBarServicesManager_Vtbl;
    const IID: ::windows::core::GUID = <IGameBarServicesManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameBarServicesManager {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManager";
}
impl ::core::convert::From<GameBarServicesManager> for ::windows::core::IUnknown {
    fn from(value: GameBarServicesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesManager> for ::windows::core::IUnknown {
    fn from(value: &GameBarServicesManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServicesManager> for ::windows::core::IInspectable {
    fn from(value: GameBarServicesManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesManager> for ::windows::core::IInspectable {
    fn from(value: &GameBarServicesManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServicesManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServicesManager {}
unsafe impl ::core::marker::Sync for GameBarServicesManager {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServicesManagerGameBarServicesCreatedEventArgs(::windows::core::IUnknown);
impl GameBarServicesManagerGameBarServicesCreatedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GameBarServices(&self) -> ::windows::core::Result<GameBarServices> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GameBarServices)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServices>(result__)
        }
    }
}
impl ::core::clone::Clone for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs;{ededbd9c-143e-49a3-a5ea-0b1995c8d46e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IGameBarServicesManagerGameBarServicesCreatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesManagerGameBarServicesCreatedEventArgs";
}
impl ::core::convert::From<GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesManagerGameBarServicesCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GameBarServicesManagerGameBarServicesCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServicesManagerGameBarServicesCreatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
unsafe impl ::core::marker::Sync for GameBarServicesManagerGameBarServicesCreatedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct GameBarServicesTargetInfo(::windows::core::IUnknown);
impl GameBarServicesTargetInfo {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AppId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn TitleId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TitleId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn DisplayMode(&self) -> ::windows::core::Result<GameBarServicesDisplayMode> {
        let this = self;
        unsafe {
            let mut result__: GameBarServicesDisplayMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GameBarServicesDisplayMode>(result__)
        }
    }
}
impl ::core::clone::Clone for GameBarServicesTargetInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for GameBarServicesTargetInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.GameBarServicesTargetInfo;{b4202f92-1611-4e05-b6ef-dfd737ae33b0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_Vtbl;
    const IID: ::windows::core::GUID = <IGameBarServicesTargetInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GameBarServicesTargetInfo {
    const NAME: &'static str = "Windows.Media.Capture.GameBarServicesTargetInfo";
}
impl ::core::convert::From<GameBarServicesTargetInfo> for ::windows::core::IUnknown {
    fn from(value: GameBarServicesTargetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesTargetInfo> for ::windows::core::IUnknown {
    fn from(value: &GameBarServicesTargetInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GameBarServicesTargetInfo> for ::windows::core::IInspectable {
    fn from(value: GameBarServicesTargetInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GameBarServicesTargetInfo> for ::windows::core::IInspectable {
    fn from(value: &GameBarServicesTargetInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GameBarServicesTargetInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GameBarServicesTargetInfo {}
unsafe impl ::core::marker::Sync for GameBarServicesTargetInfo {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for GameBarTargetCapturePolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for GameBarTargetCapturePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GameBarTargetCapturePolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GameBarTargetCapturePolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.GameBarTargetCapturePolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdvancedCapturedPhoto {
    type Vtable = IAdvancedCapturedPhoto_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf072728b_b292_4491_9d41_99807a550bbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::AdvancedPhotoMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    Mode: usize,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedCapturedPhoto2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdvancedCapturedPhoto2 {
    type Vtable = IAdvancedCapturedPhoto2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18cf6cd8_cffe_42d8_8104_017bb318f4a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedCapturedPhoto2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FrameBoundsRelativeToReferencePhoto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameBoundsRelativeToReferencePhoto: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAdvancedPhotoCapture(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAdvancedPhotoCapture {
    type Vtable = IAdvancedPhotoCapture_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83ffaafa_6667_44dc_973c_a6bce596aa0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdvancedPhotoCapture_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CaptureWithContextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureWithContextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OptionalReferencePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptionalReferencePhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOptionalReferencePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOptionalReferencePhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub AllPhotosCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AllPhotosCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAllPhotosCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAllPhotosCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundService {
    type Vtable = IAppBroadcastBackgroundService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad1e72a_fa94_46f9_95fc_d71511cda70b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetPlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastPlugInState) -> ::windows::core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT,
    pub SetSignInInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SignInInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetStreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub StreamInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TerminateBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: AppBroadcastTerminationReason, providerspecificreason: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HeartbeatRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeartbeatRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHeartbeatRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHeartbeatRequested: usize,
    pub TitleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundService2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundService2 {
    type Vtable = IAppBroadcastBackgroundService2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc8ccbbf_5549_4b87_959f_23ca401fd473);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundService2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BroadcastChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetBroadcastChannel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BroadcastTitleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BroadcastTitleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBroadcastTitleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBroadcastTitleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BroadcastLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BroadcastLanguageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBroadcastLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBroadcastLanguageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BroadcastChannelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BroadcastChannelChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBroadcastChannelChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBroadcastChannelChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundServiceSignInInfo {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e735275_88c8_4eca_89ba_4825985db880);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetOAuthRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOAuthRequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub OAuthRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthRequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetOAuthCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOAuthCallbackUri: usize,
    #[cfg(feature = "Foundation")]
    pub OAuthCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthCallbackUri: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SignInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignInStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSignInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSignInStateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundServiceSignInInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceSignInInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9104285c_62cf_4a3c_a7ee_aeb507404645);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceSignInInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub UserNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UserNameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUserNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUserNameChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundServiceStreamInfo {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31dc02bc_990a_4904_aa96_fe364381f136);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT,
    pub SetDesiredVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub DesiredVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SetBandwidthTestBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub BandwidthTestBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SetAudioCodec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AudioCodec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BroadcastStreamReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VideoEncodingResolutionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoEncodingResolutionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoEncodingResolutionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoEncodingResolutionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VideoEncodingBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoEncodingBitrateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoEncodingBitrateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoEncodingBitrateChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastBackgroundServiceStreamInfo2 {
    type Vtable = IAppBroadcastBackgroundServiceStreamInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd1e9f6d_94dc_4fce_9541_a9f129596334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastBackgroundServiceStreamInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ReportProblemWithStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastCameraCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e334cd0_b882_4b88_8692_05999aceb70f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastCameraCaptureStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastGlobalSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastGlobalSettings {
    type Vtable = IAppBroadcastGlobalSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2cb27a5_70fc_4e17_80bd_6ba0fd3ff3a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastGlobalSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsBroadcastEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetIsCameraCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCameraCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSelectedCameraId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SelectedCameraId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCameraOverlayLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlayLocation) -> ::windows::core::HRESULT,
    pub CameraOverlayLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlayLocation) -> ::windows::core::HRESULT,
    pub SetCameraOverlaySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastCameraOverlaySize) -> ::windows::core::HRESULT,
    pub CameraOverlaySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraOverlaySize) -> ::windows::core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastHeartbeatRequestedEventArgs {
    type Vtable = IAppBroadcastHeartbeatRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcea54283_ee51_4dbf_9472_79a9ed4e2165);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastHeartbeatRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastManagerStatics {
    type Vtable = IAppBroadcastManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x364e018b_1e4e_411f_ab3e_92959844c156);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetGlobalSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ApplyGlobalSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetProviderSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ApplyProviderSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa86ad5e9_9440_4908_9d09_65b7e315d795);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastMicrophoneCaptureStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugIn(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastPlugIn {
    type Vtable = IAppBroadcastPlugIn_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x520c1e66_6513_4574_ac54_23b79729615b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugIn_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProviderSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Logo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Logo: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugInManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastPlugInManager {
    type Vtable = IAppBroadcastPlugInManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe550d979_27a1_49a7_bbf4_d7a9e9d07668);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsBroadcastProviderAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PlugInList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PlugInList: usize,
    pub DefaultPlugIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDefaultPlugIn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugInManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastPlugInManagerStatics {
    type Vtable = IAppBroadcastPlugInManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2645c20_5c76_4cdc_9364_82fe9eb6534d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPlugInStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastPlugInStateChangedEventArgs {
    type Vtable = IAppBroadcastPlugInStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4881d0f2_abc5_4fc6_84b0_89370bb47212);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPlugInStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastPreview {
    type Vtable = IAppBroadcastPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14b60f5a_6e4a_4b80_a14f_67ee77d153e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StopPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PreviewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorCode: usize,
    #[cfg(feature = "Foundation")]
    pub PreviewStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviewStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreviewStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreviewStateChanged: usize,
    pub PreviewStreamReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastPreviewStateChangedEventArgs {
    type Vtable = IAppBroadcastPreviewStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a57f2de_8dea_4e86_90ad_03fc26b9653c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PreviewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPreviewState) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastPreviewStreamReader {
    type Vtable = IAppBroadcastPreviewStreamReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92228d50_db3f_40a8_8cd4_f4e371ddab37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub VideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub VideoStride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapPixelFormat: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub VideoBitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    VideoBitmapAlphaMode: usize,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub VideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoFrameArrived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastPreviewStreamVideoFrame {
    type Vtable = IAppBroadcastPreviewStreamVideoFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x010fbea1_94fe_4499_b8c0_8d244279fb12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoFrame_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub VideoHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastPreviewStreamVideoHeader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastPreviewStreamVideoHeader {
    type Vtable = IAppBroadcastPreviewStreamVideoHeader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8bef6113_da84_4499_a7ab_87118cb4a157);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastPreviewStreamVideoHeader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AbsoluteTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastProviderSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastProviderSettings {
    type Vtable = IAppBroadcastProviderSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc30bdf62_9948_458f_ad50_aa06ec03da08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastProviderSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetDefaultBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DefaultBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingBitrateMode) -> ::windows::core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastVideoEncodingResolutionMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastServices(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastServices {
    type Vtable = IAppBroadcastServices_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8660b4d6_969b_4e3c_ac3a_8b042ee4ee63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastServices_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CaptureTargetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCaptureTargetType) -> ::windows::core::HRESULT,
    pub SetCaptureTargetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastCaptureTargetType) -> ::windows::core::HRESULT,
    pub BroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetBroadcastTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetBroadcastLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CanCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnterBroadcastModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plugin: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnterBroadcastModeAsync: usize,
    pub ExitBroadcastMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: AppBroadcastExitBroadcastModeReason) -> ::windows::core::HRESULT,
    pub StartBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PauseBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResumeBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredsize: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPreview: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastSignInStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastSignInStateChangedEventArgs {
    type Vtable = IAppBroadcastSignInStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b692a4_5919_4a9e_8d5e_c9bb0dd3377a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastSignInStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT,
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInResult) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastState(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastState {
    type Vtable = IAppBroadcastState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee08056d_8099_4ddd_922e_c56dac58abfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastState_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsCaptureTargetRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShouldCaptureCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShouldCaptureCamera: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RestartCameraCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EncodedVideoSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EncodedVideoSize: usize,
    pub MicrophoneCaptureState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastMicrophoneCaptureState) -> ::windows::core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CameraCaptureState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastCameraCaptureState) -> ::windows::core::HRESULT,
    pub CameraCaptureError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub StreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT,
    pub PlugInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastPlugInState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OAuthRequestUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthRequestUri: usize,
    #[cfg(feature = "Foundation")]
    pub OAuthCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OAuthCallbackUri: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub AuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    AuthenticationResult: usize,
    #[cfg(feature = "Security_Authentication_Web")]
    pub SetAuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    SetAuthenticationResult: usize,
    pub SetSignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppBroadcastSignInState) -> ::windows::core::HRESULT,
    pub SignInState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastSignInState) -> ::windows::core::HRESULT,
    pub TerminationReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastTerminationReason) -> ::windows::core::HRESULT,
    pub TerminationReasonPlugInSpecific: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ViewerCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ViewerCountChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveViewerCountChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveViewerCountChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CameraCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlugInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlugInStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlugInStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlugInStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub StreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTargetClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCaptureTargetClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamAudioFrame {
    type Vtable = IAppBroadcastStreamAudioFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab4ac8_21ba_453f_8bb7_5e938a2e9a74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioFrame_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AudioHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamAudioHeader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamAudioHeader {
    type Vtable = IAppBroadcastStreamAudioHeader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf21a570_6b78_4216_9f07_5aff5256f1b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamAudioHeader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AbsoluteTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub HasDiscontinuity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamReader {
    type Vtable = IAppBroadcastStreamReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb338bcf9_3364_4460_b5f1_3cc2796a8aa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AudioChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AudioSampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AudioAacSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AudioAacSequence: usize,
    pub AudioBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TryGetNextAudioFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub VideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub VideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub VideoBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TryGetNextVideoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AudioFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AudioFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub VideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoFrameArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoFrameArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoFrameArrived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamStateChangedEventArgs {
    type Vtable = IAppBroadcastStreamStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5108a733_d008_4a89_93be_58aed961374e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppBroadcastStreamState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamVideoFrame {
    type Vtable = IAppBroadcastStreamVideoFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f97cf2b_c9e4_4e88_8194_d814cbd585d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoFrame_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub VideoHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub VideoBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    VideoBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastStreamVideoHeader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastStreamVideoHeader {
    type Vtable = IAppBroadcastStreamVideoHeader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b9ebece_7e32_432d_8ca2_36bf10b9f462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastStreamVideoHeader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AbsoluteTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AbsoluteTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub RelativeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RelativeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub IsKeyFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasDiscontinuity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub FrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastTriggerDetails {
    type Vtable = IAppBroadcastTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdeebab35_ec5e_4d8f_b1c0_5da6e8c75638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BackgroundService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastViewerCountChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastViewerCountChangedEventArgs {
    type Vtable = IAppBroadcastViewerCountChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6e11825_5401_4ade_8bd2_c14ecee6807d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastViewerCountChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ViewerCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapture(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCapture {
    type Vtable = IAppCapture_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9749d453_a29a_45ed_8f29_22d09942cff7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapture_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsCapturingAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCapturingVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CapturingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CapturingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCapturingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCapturingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureAlternateShortcutKeys {
    type Vtable = IAppCaptureAlternateShortcutKeys_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19e8e0ef_236c_40f9_b38f_9b7dd65d1ccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleGameBarKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleGameBarKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleGameBarKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKey: usize,
    #[cfg(feature = "System")]
    pub SetSaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetSaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SaveHistoricalVideoKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SaveHistoricalVideoKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKey: usize,
    #[cfg(feature = "System")]
    pub SetTakeScreenshotKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetTakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub TakeScreenshotKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    TakeScreenshotKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleRecordingIndicatorKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleRecordingIndicatorKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleRecordingIndicatorKeyModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureAlternateShortcutKeys2 {
    type Vtable = IAppCaptureAlternateShortcutKeys2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3669090_dd17_47f0_95e5_ce42286cf338);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleMicrophoneCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleMicrophoneCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleMicrophoneCaptureKeyModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureAlternateShortcutKeys3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureAlternateShortcutKeys3 {
    type Vtable = IAppCaptureAlternateShortcutKeys3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b81448c_418e_469c_a49a_45b597c826b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureAlternateShortcutKeys3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleCameraCaptureKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleCameraCaptureKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKey: usize,
    #[cfg(feature = "System")]
    pub SetToggleBroadcastKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetToggleBroadcastKeyModifiers: usize,
    #[cfg(feature = "System")]
    pub ToggleBroadcastKeyModifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ToggleBroadcastKeyModifiers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureDurationGeneratedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureDurationGeneratedEventArgs {
    type Vtable = IAppCaptureDurationGeneratedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1f5563b_ffa1_44c9_975f_27fbeb553b35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureDurationGeneratedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureFileGeneratedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureFileGeneratedEventArgs {
    type Vtable = IAppCaptureFileGeneratedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4189fbf4_465e_45bf_907f_165b3fb23758);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureFileGeneratedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureManagerStatics {
    type Vtable = IAppCaptureManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d9e3ea7_6282_4735_8d4e_aa45f90f6723);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ApplySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appcapturesettings: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureMetadataWriter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureMetadataWriter {
    type Vtable = IAppCaptureMetadataWriter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0ce4877_9aaf_46b4_ad31_6a60b441c780);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMetadataWriter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AddStringEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub AddInt32Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub AddDoubleEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub StartStringState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub StartInt32State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: i32, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub StartDoubleState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: f64, priority: AppCaptureMetadataPriority) -> ::windows::core::HRESULT,
    pub StopState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StopAllStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemainingStorageBytesAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MetadataPurged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MetadataPurged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMetadataPurged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMetadataPurged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureMicrophoneCaptureStateChangedEventArgs {
    type Vtable = IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x324d249e_45bc_4c35_bc35_e469fc7a69e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureMicrophoneCaptureStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureRecordOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureRecordOperation {
    type Vtable = IAppCaptureRecordOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc66020a9_1538_495c_9bbb_2ba870ec5861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StopRecording: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorCode: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Foundation")]
    pub IsFileTruncated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsFileTruncated: usize,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DurationGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDurationGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDurationGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub FileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileGenerated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileGenerated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureRecordingStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureRecordingStateChangedEventArgs {
    type Vtable = IAppCaptureRecordingStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24fc8712_e305_490d_b415_6b1c9049736b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureRecordingStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureRecordingState) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureServices(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureServices {
    type Vtable = IAppCaptureServices_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44fec0b5_34f5_4f18_ae8c_b9123abbfc0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureServices_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Record: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordTimeSpan: usize,
    pub CanCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureSettings {
    type Vtable = IAppCaptureSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14683a86_8807_48d3_883a_970ee4532a39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage")]
    pub SetAppCaptureDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetAppCaptureDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub AppCaptureDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    AppCaptureDestinationFolder: usize,
    pub SetAudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub AudioEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetIsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsAudioCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CustomVideoEncodingBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CustomVideoEncodingHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CustomVideoEncodingWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetHistoricalBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub HistoricalBufferLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetHistoricalBufferLengthUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::HRESULT,
    pub HistoricalBufferLengthUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureHistoricalBufferLengthUnit) -> ::windows::core::HRESULT,
    pub SetIsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsHistoricalCaptureOnBatteryAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsHistoricalCaptureOnWirelessDisplayAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetMaximumRecordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaximumRecordLength: usize,
    #[cfg(feature = "Foundation")]
    pub MaximumRecordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaximumRecordLength: usize,
    #[cfg(feature = "Storage")]
    pub SetScreenshotDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetScreenshotDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub ScreenshotDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    ScreenshotDestinationFolder: usize,
    pub SetVideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingBitrateMode) -> ::windows::core::HRESULT,
    pub VideoEncodingBitrateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingBitrateMode) -> ::windows::core::HRESULT,
    pub SetVideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingResolutionMode) -> ::windows::core::HRESULT,
    pub VideoEncodingResolutionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingResolutionMode) -> ::windows::core::HRESULT,
    pub SetIsAppCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsAppCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDisabledByPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsMemoryConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasHardwareEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureSettings2 {
    type Vtable = IAppCaptureSettings2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcb8cee7_e26b_476f_9b1a_ec342d2a8fde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AlternateShortcutKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureSettings3 {
    type Vtable = IAppCaptureSettings3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa93502fe_88c2_42d6_aaaa_40feffd75aec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetIsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsMicrophoneCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureSettings4 {
    type Vtable = IAppCaptureSettings4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07c2774c_1a81_482f_a244_049d95f25b0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetIsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsMicrophoneCaptureEnabledByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub SystemAudioGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetMicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub MicrophoneGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetVideoEncodingFrameRateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::HRESULT,
    pub VideoEncodingFrameRateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureVideoEncodingFrameRateMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureSettings5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureSettings5 {
    type Vtable = IAppCaptureSettings5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18894522_b0e8_4ba0_8f13_3eaa5fa4013b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureSettings5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetIsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsEchoCancellationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCursorImageCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureState(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureState {
    type Vtable = IAppCaptureState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73134372_d4eb_44ce_9538_465f506ac4ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureState_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsTargetRunning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHistoricalCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShouldCaptureMicrophone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RestartMicrophoneCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MicrophoneCaptureState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCaptureMicrophoneCaptureState) -> ::windows::core::HRESULT,
    pub MicrophoneCaptureError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMicrophoneCaptureStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMicrophoneCaptureStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTargetClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCaptureTargetClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCaptureTargetClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureStatics {
    type Vtable = IAppCaptureStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf922dd6c_0a7e_4e74_8b20_9c1f902d08a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCaptureStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCaptureStatics2 {
    type Vtable = IAppCaptureStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2d881d4_836c_4da4_afd7_facc041e1cf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCaptureStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetAllowedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allowed: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAllowedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraCaptureUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraCaptureUI {
    type Vtable = ICameraCaptureUI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48587540_6f93_4bb4_b8f3_e89e48948c91);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUI_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PhotoSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub VideoSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CaptureFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: CameraCaptureUIMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CaptureFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraCaptureUIPhotoCaptureSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraCaptureUIPhotoCaptureSettings {
    type Vtable = ICameraCaptureUIPhotoCaptureSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9f5be97_3472_46a8_8a9e_04ce42ccc97d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIPhotoCaptureSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIPhotoFormat) -> ::windows::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIPhotoFormat) -> ::windows::core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxPhotoResolution) -> ::windows::core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxPhotoResolution) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CroppedSizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CroppedSizeInPixels: usize,
    #[cfg(feature = "Foundation")]
    pub SetCroppedSizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCroppedSizeInPixels: usize,
    #[cfg(feature = "Foundation")]
    pub CroppedAspectRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CroppedAspectRatio: usize,
    #[cfg(feature = "Foundation")]
    pub SetCroppedAspectRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCroppedAspectRatio: usize,
    pub AllowCropping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowCropping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraCaptureUIVideoCaptureSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraCaptureUIVideoCaptureSettings {
    type Vtable = ICameraCaptureUIVideoCaptureSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64e92d1f_a28d_425a_b84f_e568335ff24e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraCaptureUIVideoCaptureSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIVideoFormat) -> ::windows::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIVideoFormat) -> ::windows::core::HRESULT,
    pub MaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CameraCaptureUIMaxVideoResolution) -> ::windows::core::HRESULT,
    pub SetMaxResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CameraCaptureUIMaxVideoResolution) -> ::windows::core::HRESULT,
    pub MaxDurationInSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetMaxDurationInSeconds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub AllowTrimming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowTrimming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraOptionsUIStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraOptionsUIStatics {
    type Vtable = ICameraOptionsUIStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b0d5e34_3906_4b7d_946c_7bde844499ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraOptionsUIStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediacapture: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICapturedFrame {
    type Vtable = ICapturedFrame_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dd2de1f_571b_44d8_8e80_a08a1578766e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrame2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICapturedFrame2 {
    type Vtable = ICapturedFrame2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x543fa6d1_bd78_4866_adda_24314bc65dea);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrame2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ControlValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub BitmapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    BitmapProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrameControlValues(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICapturedFrameControlValues {
    type Vtable = ICapturedFrameControlValues_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90c65b7f_4e0d_4ca4_882d_7a144fed0a90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Exposure: usize,
    #[cfg(feature = "Foundation")]
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExposureCompensation: usize,
    #[cfg(feature = "Foundation")]
    pub IsoSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsoSpeed: usize,
    #[cfg(feature = "Foundation")]
    pub Focus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Focus: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub SceneMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    SceneMode: usize,
    #[cfg(feature = "Foundation")]
    pub Flashed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Flashed: usize,
    #[cfg(feature = "Foundation")]
    pub FlashPowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlashPowerPercent: usize,
    #[cfg(feature = "Foundation")]
    pub WhiteBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WhiteBalance: usize,
    #[cfg(feature = "Foundation")]
    pub ZoomFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ZoomFactor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrameControlValues2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICapturedFrameControlValues2 {
    type Vtable = ICapturedFrameControlValues2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x500b2b88_06d2_4aa7_a7db_d37af73321d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameControlValues2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    FocusState: usize,
    #[cfg(feature = "Foundation")]
    pub IsoDigitalGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsoDigitalGain: usize,
    #[cfg(feature = "Foundation")]
    pub IsoAnalogGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsoAnalogGain: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SensorFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SensorFrameRate: usize,
    #[cfg(feature = "Foundation")]
    pub WhiteBalanceGain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WhiteBalanceGain: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedFrameWithSoftwareBitmap(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICapturedFrameWithSoftwareBitmap {
    type Vtable = ICapturedFrameWithSoftwareBitmap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb58e8b6e_8503_49b5_9e86_897d26a3ff3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedFrameWithSoftwareBitmap_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICapturedPhoto(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICapturedPhoto {
    type Vtable = ICapturedPhoto_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0ce7e5a_cfcc_4d6c_8ad1_0869208aca16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICapturedPhoto_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServices(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameBarServices {
    type Vtable = IGameBarServices_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dbead57_50a6_499e_8c6c_d330a7311796);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServices_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TargetCapturePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarTargetCapturePolicy) -> ::windows::core::HRESULT,
    pub EnableCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableCapture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TargetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppBroadcastServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AppCaptureServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CommandReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CommandReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCommandReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCommandReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesCommandEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameBarServicesCommandEventArgs {
    type Vtable = IGameBarServicesCommandEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74226b2_f176_4fcf_8fbb_cf698b2eb8e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesCommandEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommand) -> ::windows::core::HRESULT,
    pub Origin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarCommandOrigin) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameBarServicesManager {
    type Vtable = IGameBarServicesManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a4b9cfa_7f8b_4c60_9dbb_0bcd262dffc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GameBarServicesCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GameBarServicesCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGameBarServicesCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGameBarServicesCreated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameBarServicesManagerGameBarServicesCreatedEventArgs {
    type Vtable = IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xededbd9c_143e_49a3_a5ea_0b1995c8d46e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerGameBarServicesCreatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GameBarServices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameBarServicesManagerStatics {
    type Vtable = IGameBarServicesManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34c1b616_ff25_4792_98f2_d3753f15ac13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGameBarServicesTargetInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGameBarServicesTargetInfo {
    type Vtable = IGameBarServicesTargetInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4202f92_1611_4e05_b6ef_dfd737ae33b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGameBarServicesTargetInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TitleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GameBarServicesDisplayMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagMediaRecording(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41c8baf7_ff3f_49f0_a477_f195e3ce5108);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagMediaRecording2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLagMediaRecording2 {
    type Vtable = ILowLagMediaRecording2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6369c758_5644_41e2_97af_8ef56a25e225);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagMediaRecording3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLagMediaRecording3 {
    type Vtable = ILowLagMediaRecording3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c33ab12_48f7_47da_b41e_90880a5fe0ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagMediaRecording3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseWithResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopWithResultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoCapture(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa37251b7_6b44_473d_8f24_f703d6c0ec44);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoCapture_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLagPhotoSequenceCapture(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cc346bb_b9a9_4c91_8ffa_287e9c668669);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLagPhotoSequenceCapture_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePhotoCaptured: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCapture {
    type Vtable = IMediaCapture_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc61afbb4_fb10_4a34_ac18_ca80d9c8e7ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub InitializeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitializeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub InitializeWithSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediacaptureinitializationsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitializeWithSettingsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub StartRecordToStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    StartRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub StartRecordToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    StartRecordToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub StartRecordToCustomSinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    StartRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartRecordToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartRecordToCustomSinkIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRecordAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub CapturePhotoToStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    CapturePhotoToStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub CapturePhotoToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    CapturePhotoToStreamAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub AddEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, effectactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, effectsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    AddEffectAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClearEffectsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearEffectsAsync: usize,
    pub SetEncoderProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows::core::GUID, propertyvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEncoderProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, propertyid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Failed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, erroreventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Failed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RecordLimitationExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recordlimitationexceededeventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordLimitationExceeded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRecordLimitationExceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRecordLimitationExceeded: usize,
    pub MediaCaptureSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Devices")]
    pub AudioDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    AudioDeviceController: usize,
    #[cfg(feature = "Media_Devices")]
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    VideoDeviceController: usize,
    pub SetPreviewMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub GetPreviewMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPreviewRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows::core::HRESULT,
    pub GetPreviewRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows::core::HRESULT,
    pub SetRecordRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VideoRotation) -> ::windows::core::HRESULT,
    pub GetRecordRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoRotation) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCapture2 {
    type Vtable = IMediaCapture2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cc68260_7da1_4043_b652_21b8878daff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub PrepareLowLagRecordToStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage")))]
    PrepareLowLagRecordToStorageFileAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub PrepareLowLagRecordToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams")))]
    PrepareLowLagRecordToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareLowLagRecordToCustomSinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareLowLagRecordToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub PrepareLowLagRecordToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    PrepareLowLagRecordToCustomSinkIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareLowLagPhotoCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareLowLagPhotoCaptureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareLowLagPhotoSequenceCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareLowLagPhotoSequenceCaptureAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub SetEncodingPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediastreamtype: MediaStreamType, mediaencodingproperties: ::windows::core::RawPtr, encoderproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    SetEncodingPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCapture3 {
    type Vtable = IMediaCapture3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4136f30_1564_466e_bc0a_af94e02ab016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub PrepareVariablePhotoSequenceCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties")))]
    PrepareVariablePhotoSequenceCaptureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FocusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FocusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFocusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFocusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PhotoConfirmationCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoConfirmationCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePhotoConfirmationCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePhotoConfirmationCaptured: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCapture4 {
    type Vtable = IMediaCapture4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbacd6fd6_fb08_4947_aea2_ce14eff0ce13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub AddAudioEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))]
    AddAudioEffectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub AddVideoEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, definition: ::windows::core::RawPtr, mediastreamtype: MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Effects")))]
    AddVideoEffectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseRecordAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ResumeRecordAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResumeRecordAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CameraStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CameraStreamStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCameraStreamStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCameraStreamStateChanged: usize,
    #[cfg(feature = "Media_Devices")]
    pub CameraStreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::CameraStreamState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    CameraStreamState: usize,
    #[cfg(feature = "Foundation")]
    pub GetPreviewFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPreviewFrameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPreviewFrameCopyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPreviewFrameCopyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ThermalStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ThermalStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveThermalStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveThermalStatusChanged: usize,
    pub ThermalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureThermalStatus) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub PrepareAdvancedPhotoCaptureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    PrepareAdvancedPhotoCaptureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCapture5 {
    type Vtable = IMediaCapture5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda787c22_3a9b_4720_a71e_97900a316e5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RemoveEffectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEffectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub PauseRecordWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, behavior: super::Devices::MediaCapturePauseBehavior, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Devices")))]
    PauseRecordWithResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopRecordWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopRecordWithResultAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSources: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))]
    CreateFrameReaderAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderWithSubtypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsource: ::windows::core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Capture_Frames")))]
    CreateFrameReaderWithSubtypeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub CreateFrameReaderWithSubtypeAndSizeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsource: ::windows::core::RawPtr, outputsubtype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, outputsize: super::super::Graphics::Imaging::BitmapSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames")))]
    CreateFrameReaderWithSubtypeAndSizeAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCapture6 {
    type Vtable = IMediaCapture6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x228948bd_4b20_4bb1_9fd6_a583212a1012);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureDeviceExclusiveControlStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCaptureDeviceExclusiveControlStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCaptureDeviceExclusiveControlStatusChanged: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub CreateMultiSourceFrameReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputsources: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    CreateMultiSourceFrameReaderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapture7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCapture7 {
    type Vtable = IMediaCapture7_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9169f102_8888_541a_95bc_24e4d462542a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapture7_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub CreateRelativePanelWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capturemode: StreamingCaptureMode, displayregion: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    CreateRelativePanelWatcher: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d2f920d_a588_43c6_89d6_5ad322af006a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureDeviceExclusiveControlStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80fde3f4_54c4_42c0_8d19_cea1a87ca18b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFailedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Code: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureFocusChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81e1bc7f_2277_493e_abee_d3f44ff98c04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureFocusChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media_Devices")]
    pub FocusState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Devices::MediaCaptureFocusState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Devices"))]
    FocusState: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9782ba70_ea65_4900_9356_8ca887726884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetAudioDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AudioDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetVideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetStreamingCaptureMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StreamingCaptureMode) -> ::windows::core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows::core::HRESULT,
    pub SetPhotoCaptureSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhotoCaptureSource) -> ::windows::core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings2 {
    type Vtable = IMediaCaptureInitializationSettings2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x404e0626_c9dc_43e9_aee4_e6bf1b57b44c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetMediaCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCategory) -> ::windows::core::HRESULT,
    pub MediaCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows::core::HRESULT,
    pub SetAudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::AudioProcessing) -> ::windows::core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings3 {
    type Vtable = IMediaCaptureInitializationSettings3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4160519d_be48_4730_8104_0cf6e9e97948);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media_Core")]
    pub SetAudioSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetAudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub SetVideoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    SetVideoSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings4 {
    type Vtable = IMediaCaptureInitializationSettings4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf502a537_4cb7_4d28_95ed_4f9f012e0518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub VideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetVideoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PreviewMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPreviewMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RecordMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRecordMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PhotoMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPhotoMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings5 {
    type Vtable = IMediaCaptureInitializationSettings5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd5a2e3b8_2626_4e94_b7b3_5308a0f64b1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings5_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SourceGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SourceGroup: usize,
    #[cfg(feature = "Media_Capture_Frames")]
    pub SetSourceGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    SetSourceGroup: usize,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureSharingMode) -> ::windows::core::HRESULT,
    pub SetSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCaptureSharingMode) -> ::windows::core::HRESULT,
    pub MemoryPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCaptureMemoryPreference) -> ::windows::core::HRESULT,
    pub SetMemoryPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCaptureMemoryPreference) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings6 {
    type Vtable = IMediaCaptureInitializationSettings6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2e26b47_3db1_4d33_ab63_0ffa09056585);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings6_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAlwaysPlaySystemShutterSound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureInitializationSettings7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureInitializationSettings7 {
    type Vtable = IMediaCaptureInitializationSettings7_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41546967_f58a_5d82_9ef4_ed572fb5e34e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureInitializationSettings7_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub DeviceUriPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    DeviceUriPasswordCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetDeviceUriPasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetDeviceUriPasswordCredential: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetDeviceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDeviceUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCapturePauseResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaec47ca3_4477_4b04_a06f_2c1c5182fe9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCapturePauseResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LastFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureRelativePanelWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d896566_04be_5b89_b30e_bd34a9f12db0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureRelativePanelWatcher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub RelativePanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Enumeration::Panel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    RelativePanel: usize,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d83aafe_6d45_4477_8dc4_ac5bc01c4091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AudioDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StreamingCaptureMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StreamingCaptureMode) -> ::windows::core::HRESULT,
    pub PhotoCaptureSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoCaptureSource) -> ::windows::core::HRESULT,
    pub VideoDeviceCharacteristic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoDeviceCharacteristic) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureSettings2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureSettings2 {
    type Vtable = IMediaCaptureSettings2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f9e7cfb_fa9f_4b13_9cbe_5ab94f1f3493);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ConcurrentRecordAndPhotoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ConcurrentRecordAndPhotoSequenceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CameraSoundRequiredForRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Horizontal35mmEquivalentFocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Horizontal35mmEquivalentFocalLength: usize,
    #[cfg(feature = "Foundation")]
    pub PitchOffsetDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PitchOffsetDegrees: usize,
    #[cfg(feature = "Foundation")]
    pub Vertical35mmEquivalentFocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Vertical35mmEquivalentFocalLength: usize,
    pub MediaCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCategory) -> ::windows::core::HRESULT,
    pub AudioProcessing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AudioProcessing) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureSettings3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureSettings3 {
    type Vtable = IMediaCaptureSettings3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x303c67c2_8058_4b1b_b877_8c2ef3528440);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureSettings3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Device: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureStatics {
    type Vtable = IMediaCaptureStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacef81ff_99ed_4645_965e_1925cfc63834);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsVideoProfileSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllVideoProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConcurrentProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConcurrentProfiles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindKnownVideoProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, videodeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: KnownVideoProfile, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindKnownVideoProfiles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureStopResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9db6a2a_a092_4ad1_97d4_f201f9d082db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureStopResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LastFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RecordDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecordDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoPreview {
    type Vtable = IMediaCaptureVideoPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27727073_549e_447f_a20a_4f03c479d8c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoPreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPreviewAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub StartPreviewToCustomSinkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, custommediasink: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_MediaProperties")))]
    StartPreviewToCustomSinkAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub StartPreviewToCustomSinkIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingprofile: ::windows::core::RawPtr, customsinkactivationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, customsinksettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_MediaProperties")))]
    StartPreviewToCustomSinkIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopPreviewAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21a073bf_a3ee_4ecf_9ef6_50b0bc4e1305);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPreviewMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPreviewMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedRecordMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedRecordMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPhotoMediaDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPhotoMediaDescription: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConcurrency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConcurrency: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfile2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoProfile2 {
    type Vtable = IMediaCaptureVideoProfile2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97ddc95f_94ce_468f_9316_fc5bc2638f6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfile2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub FrameSourceInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames")))]
    FrameSourceInfos: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8012afef_b691_49ff_83f2_c1e76eaaea1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub FrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsVariablePhotoSequenceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsVariablePhotoSequenceSupported: usize,
    #[cfg(feature = "deprecated")]
    pub IsHdrVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsHdrVideoSupported: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCaptureVideoProfileMediaDescription2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaCaptureVideoProfileMediaDescription2 {
    type Vtable = IMediaCaptureVideoProfileMediaDescription2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6a6ef13_322d_413a_b85a_68a88e02f4e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCaptureVideoProfileMediaDescription2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Subtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOptionalReferencePhotoCapturedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x470f88b3_1e6d_4051_9c8b_f1d85af047b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOptionalReferencePhotoCapturedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoCapturedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x373bfbc1_984e_4ff0_bf85_1c00aabc5a45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoCapturedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoConfirmationCapturedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab473672_c28a_4827_8f8d_3636d3beb51e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoConfirmationCapturedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenCapture(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IScreenCapture {
    type Vtable = IScreenCapture_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89179ef7_cd12_4e0e_a6d4_5b3de98b2e9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCapture_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media_Core")]
    pub AudioSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    AudioSource: usize,
    #[cfg(feature = "Media_Core")]
    pub VideoSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    VideoSource: usize,
    pub IsAudioSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SourceSuspensionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceSuspensionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceSuspensionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceSuspensionChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IScreenCaptureStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IScreenCaptureStatics {
    type Vtable = IScreenCaptureStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc898c3b0_c8a5_11e2_8b8b_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScreenCaptureStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISourceSuspensionChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ece7b5e_d49b_4394_bc32_f97d6cedec1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISourceSuspensionChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsAudioSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVideoSuspended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8770a6f_4390_4b5e_ad3e_0f8af0963490);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub InputProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    InputProperties: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub OutputProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    OutputProperties: usize,
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for KnownVideoProfile {
    type Abi = Self;
}
impl ::core::fmt::Debug for KnownVideoProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KnownVideoProfile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KnownVideoProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.KnownVideoProfile;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct LowLagMediaRecording(::windows::core::IUnknown);
impl LowLagMediaRecording {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FinishAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PauseAsync)(::core::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ILowLagMediaRecording2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResumeAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows::core::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PauseWithResultAsync)(::core::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopWithResultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows::core::Interface::cast::<ILowLagMediaRecording3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopWithResultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for LowLagMediaRecording {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LowLagMediaRecording {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagMediaRecording;{41c8baf7-ff3f-49f0-a477-f195e3ce5108})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LowLagMediaRecording {
    type Vtable = ILowLagMediaRecording_Vtbl;
    const IID: ::windows::core::GUID = <ILowLagMediaRecording as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LowLagMediaRecording {
    const NAME: &'static str = "Windows.Media.Capture.LowLagMediaRecording";
}
impl ::core::convert::From<LowLagMediaRecording> for ::windows::core::IUnknown {
    fn from(value: LowLagMediaRecording) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagMediaRecording> for ::windows::core::IUnknown {
    fn from(value: &LowLagMediaRecording) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLagMediaRecording {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLagMediaRecording {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLagMediaRecording> for ::windows::core::IInspectable {
    fn from(value: LowLagMediaRecording) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagMediaRecording> for ::windows::core::IInspectable {
    fn from(value: &LowLagMediaRecording) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLagMediaRecording {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLagMediaRecording {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct LowLagPhotoCapture(::windows::core::IUnknown);
impl LowLagPhotoCapture {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CapturedPhoto>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CapturedPhoto>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FinishAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for LowLagPhotoCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LowLagPhotoCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoCapture;{a37251b7-6b44-473d-8f24-f703d6c0ec44})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LowLagPhotoCapture {
    type Vtable = ILowLagPhotoCapture_Vtbl;
    const IID: ::windows::core::GUID = <ILowLagPhotoCapture as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LowLagPhotoCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoCapture";
}
impl ::core::convert::From<LowLagPhotoCapture> for ::windows::core::IUnknown {
    fn from(value: LowLagPhotoCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoCapture> for ::windows::core::IUnknown {
    fn from(value: &LowLagPhotoCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLagPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLagPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLagPhotoCapture> for ::windows::core::IInspectable {
    fn from(value: LowLagPhotoCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoCapture> for ::windows::core::IInspectable {
    fn from(value: &LowLagPhotoCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLagPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLagPhotoCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct LowLagPhotoSequenceCapture(::windows::core::IUnknown);
impl LowLagPhotoSequenceCapture {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FinishAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhotoCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<LowLagPhotoSequenceCapture, PhotoCapturedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhotoCaptured)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePhotoCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePhotoCaptured)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for LowLagPhotoSequenceCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LowLagPhotoSequenceCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.LowLagPhotoSequenceCapture;{7cc346bb-b9a9-4c91-8ffa-287e9c668669})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LowLagPhotoSequenceCapture {
    type Vtable = ILowLagPhotoSequenceCapture_Vtbl;
    const IID: ::windows::core::GUID = <ILowLagPhotoSequenceCapture as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LowLagPhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.LowLagPhotoSequenceCapture";
}
impl ::core::convert::From<LowLagPhotoSequenceCapture> for ::windows::core::IUnknown {
    fn from(value: LowLagPhotoSequenceCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceCapture> for ::windows::core::IUnknown {
    fn from(value: &LowLagPhotoSequenceCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLagPhotoSequenceCapture> for ::windows::core::IInspectable {
    fn from(value: LowLagPhotoSequenceCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLagPhotoSequenceCapture> for ::windows::core::IInspectable {
    fn from(value: &LowLagPhotoSequenceCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLagPhotoSequenceCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCapture(::windows::core::IUnknown);
impl MediaCapture {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaCapture, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitializeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InitializeAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitializeWithSettingsAsync<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureInitializationSettings>>(&self, mediacaptureinitializationsettings: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InitializeWithSettingsAsync)(::core::mem::transmute_copy(this), mediacaptureinitializationsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn StartRecordToStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, encodingprofile: Param0, file: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartRecordToStorageFileAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn StartRecordToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, encodingprofile: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartRecordToStreamAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn StartRecordToCustomSinkAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartRecordToCustomSinkAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartRecordToCustomSinkIdAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartRecordToCustomSinkIdAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopRecordAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn CapturePhotoToStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, r#type: Param0, file: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CapturePhotoToStorageFileAsync)(::core::mem::transmute_copy(this), r#type.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn CapturePhotoToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, r#type: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CapturePhotoToStreamAsync)(::core::mem::transmute_copy(this), r#type.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn AddEffectAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, mediastreamtype: MediaStreamType, effectactivationid: Param1, effectsettings: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddEffectAsync)(::core::mem::transmute_copy(this), mediastreamtype, effectactivationid.into_param().abi(), effectsettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearEffectsAsync(&self, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearEffectsAsync)(::core::mem::transmute_copy(this), mediastreamtype, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetEncoderProperty<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, mediastreamtype: MediaStreamType, propertyid: Param1, propertyvalue: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEncoderProperty)(::core::mem::transmute_copy(this), mediastreamtype, propertyid.into_param().abi(), propertyvalue.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetEncoderProperty<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, mediastreamtype: MediaStreamType, propertyid: Param1) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetEncoderProperty)(::core::mem::transmute_copy(this), mediastreamtype, propertyid.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Failed<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureFailedEventHandler>>(&self, erroreventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Failed)(::core::mem::transmute_copy(this), erroreventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFailed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFailed)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecordLimitationExceeded<'a, Param0: ::windows::core::IntoParam<'a, RecordLimitationExceededEventHandler>>(&self, recordlimitationexceededeventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordLimitationExceeded)(::core::mem::transmute_copy(this), recordlimitationexceededeventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRecordLimitationExceeded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRecordLimitationExceeded)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MediaCaptureSettings(&self) -> ::windows::core::Result<MediaCaptureSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaCaptureSettings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn AudioDeviceController(&self) -> ::windows::core::Result<super::Devices::AudioDeviceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioDeviceController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::AudioDeviceController>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<super::Devices::VideoDeviceController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceController)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::VideoDeviceController>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetPreviewMirroring(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreviewMirroring)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetPreviewMirroring(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPreviewMirroring)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetPreviewRotation(&self, value: VideoRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreviewRotation)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetPreviewRotation(&self) -> ::windows::core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__: VideoRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPreviewRotation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoRotation>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetRecordRotation(&self, value: VideoRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRecordRotation)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetRecordRotation(&self) -> ::windows::core::Result<VideoRotation> {
        let this = self;
        unsafe {
            let mut result__: VideoRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRecordRotation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoRotation>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage"))]
    pub fn PrepareLowLagRecordToStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, encodingprofile: Param0, file: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareLowLagRecordToStorageFileAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties", feature = "Storage_Streams"))]
    pub fn PrepareLowLagRecordToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, encodingprofile: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareLowLagRecordToStreamAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagRecordToCustomSinkAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareLowLagRecordToCustomSinkAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagRecordToCustomSinkIdAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareLowLagRecordToCustomSinkIdAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagMediaRecording>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagPhotoCaptureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareLowLagPhotoCaptureAsync)(::core::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagPhotoCapture>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareLowLagPhotoSequenceCaptureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareLowLagPhotoSequenceCaptureAsync)(::core::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LowLagPhotoSequenceCapture>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn SetEncodingPropertiesAsync<'a, Param1: ::windows::core::IntoParam<'a, super::MediaProperties::IMediaEncodingProperties>, Param2: ::windows::core::IntoParam<'a, super::MediaProperties::MediaPropertySet>>(&self, mediastreamtype: MediaStreamType, mediaencodingproperties: Param1, encoderproperties: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCapture2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetEncodingPropertiesAsync)(::core::mem::transmute_copy(this), mediastreamtype, mediaencodingproperties.into_param().abi(), encoderproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Capture_Core\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Core", feature = "Media_MediaProperties"))]
    pub fn PrepareVariablePhotoSequenceCaptureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, r#type: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareVariablePhotoSequenceCaptureAsync)(::core::mem::transmute_copy(this), r#type.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Core::VariablePhotoSequenceCapture>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FocusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureFocusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFocusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFocusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhotoConfirmationCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, PhotoConfirmationCapturedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhotoConfirmationCaptured)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePhotoConfirmationCaptured<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePhotoConfirmationCaptured)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub fn AddAudioEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IAudioEffectDefinition>>(&self, definition: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddAudioEffectAsync)(::core::mem::transmute_copy(this), definition.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Effects"))]
    pub fn AddVideoEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Effects::IVideoEffectDefinition>>(&self, definition: Param0, mediastreamtype: MediaStreamType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IMediaExtension>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddVideoEffectAsync)(::core::mem::transmute_copy(this), definition.into_param().abi(), mediastreamtype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::IMediaExtension>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseRecordAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PauseRecordAsync)(::core::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResumeRecordAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResumeRecordAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CameraStreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraStreamStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCameraStreamStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCameraStreamStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn CameraStreamState(&self) -> ::windows::core::Result<super::Devices::CameraStreamState> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: super::Devices::CameraStreamState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraStreamState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::CameraStreamState>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPreviewFrameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPreviewFrameAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::VideoFrame>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPreviewFrameCopyAsync<'a, Param0: ::windows::core::IntoParam<'a, super::VideoFrame>>(&self, destination: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::VideoFrame>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPreviewFrameCopyAsync)(::core::mem::transmute_copy(this), destination.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::VideoFrame>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThermalStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThermalStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThermalStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveThermalStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ThermalStatus(&self) -> ::windows::core::Result<MediaCaptureThermalStatus> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: MediaCaptureThermalStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ThermalStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureThermalStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn PrepareAdvancedPhotoCaptureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::ImageEncodingProperties>>(&self, encodingproperties: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrepareAdvancedPhotoCaptureAsync)(::core::mem::transmute_copy(this), encodingproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AdvancedPhotoCapture>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEffectAsync<'a, Param0: ::windows::core::IntoParam<'a, super::IMediaExtension>>(&self, effect: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveEffectAsync)(::core::mem::transmute_copy(this), effect.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Devices\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Devices"))]
    pub fn PauseRecordWithResultAsync(&self, behavior: super::Devices::MediaCapturePauseBehavior) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PauseRecordWithResultAsync)(::core::mem::transmute_copy(this), behavior, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCapturePauseResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopRecordWithResultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopRecordWithResultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MediaCaptureStopResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, Frames::MediaFrameSource>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameSources)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, Frames::MediaFrameSource>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderAsync<'a, Param0: ::windows::core::IntoParam<'a, Frames::MediaFrameSource>>(&self, inputsource: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrameReaderAsync)(::core::mem::transmute_copy(this), inputsource.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderWithSubtypeAsync<'a, Param0: ::windows::core::IntoParam<'a, Frames::MediaFrameSource>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, inputsource: Param0, outputsubtype: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrameReaderWithSubtypeAsync)(::core::mem::transmute_copy(this), inputsource.into_param().abi(), outputsubtype.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Graphics_Imaging\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "Media_Capture_Frames"))]
    pub fn CreateFrameReaderWithSubtypeAndSizeAsync<'a, Param0: ::windows::core::IntoParam<'a, Frames::MediaFrameSource>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapSize>>(&self, inputsource: Param0, outputsubtype: Param1, outputsize: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFrameReaderWithSubtypeAndSizeAsync)(::core::mem::transmute_copy(this), inputsource.into_param().abi(), outputsubtype.into_param().abi(), outputsize.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MediaFrameReader>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureDeviceExclusiveControlStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCapture, MediaCaptureDeviceExclusiveControlStatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureDeviceExclusiveControlStatusChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCaptureDeviceExclusiveControlStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCaptureDeviceExclusiveControlStatusChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn CreateMultiSourceFrameReaderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<Frames::MediaFrameSource>>>(&self, inputsources: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>> {
        let this = &::windows::core::Interface::cast::<IMediaCapture6>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateMultiSourceFrameReaderAsync)(::core::mem::transmute_copy(this), inputsources.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Frames::MultiSourceMediaFrameReader>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn CreateRelativePanelWatcher<'a, Param1: ::windows::core::IntoParam<'a, super::super::UI::WindowManagement::DisplayRegion>>(&self, capturemode: StreamingCaptureMode, displayregion: Param1) -> ::windows::core::Result<MediaCaptureRelativePanelWatcher> {
        let this = &::windows::core::Interface::cast::<IMediaCapture7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateRelativePanelWatcher)(::core::mem::transmute_copy(this), capturemode, displayregion.into_param().abi(), &mut result__).from_abi::<MediaCaptureRelativePanelWatcher>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsVideoProfileSupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(videodeviceid: Param0) -> ::windows::core::Result<bool> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVideoProfileSupported)(::core::mem::transmute_copy(this), videodeviceid.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllVideoProfiles<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(videodeviceid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllVideoProfiles)(::core::mem::transmute_copy(this), videodeviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindConcurrentProfiles<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(videodeviceid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindConcurrentProfiles)(::core::mem::transmute_copy(this), videodeviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindKnownVideoProfiles<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(videodeviceid: Param0, name: KnownVideoProfile) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        Self::IMediaCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindKnownVideoProfiles)(::core::mem::transmute_copy(this), videodeviceid.into_param().abi(), name, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartPreviewAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_MediaProperties"))]
    pub fn StartPreviewToCustomSinkAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, super::IMediaExtension>>(&self, encodingprofile: Param0, custommediasink: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartPreviewToCustomSinkAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), custommediasink.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_MediaProperties"))]
    pub fn StartPreviewToCustomSinkIdAsync<'a, Param0: ::windows::core::IntoParam<'a, super::MediaProperties::MediaEncodingProfile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(&self, encodingprofile: Param0, customsinkactivationid: Param1, customsinksettings: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartPreviewToCustomSinkIdAsync)(::core::mem::transmute_copy(this), encodingprofile.into_param().abi(), customsinkactivationid.into_param().abi(), customsinksettings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoPreview>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopPreviewAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IMediaCaptureStatics<R, F: FnOnce(&IMediaCaptureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaCapture, IMediaCaptureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MediaCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapture;{c61afbb4-fb10-4a34-ac18-ca80d9c8e7ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCapture {
    type Vtable = IMediaCapture_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCapture as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCapture {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapture";
}
impl ::core::convert::From<MediaCapture> for ::windows::core::IUnknown {
    fn from(value: MediaCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCapture> for ::windows::core::IUnknown {
    fn from(value: &MediaCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCapture> for ::windows::core::IInspectable {
    fn from(value: MediaCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCapture> for ::windows::core::IInspectable {
    fn from(value: &MediaCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaCapture> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaCapture) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaCapture> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaCapture) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCapture {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaCaptureDeviceExclusiveControlStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureDeviceExclusiveControlStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureDeviceExclusiveControlStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureDeviceExclusiveControlStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureDeviceExclusiveControlStatusChangedEventArgs(::windows::core::IUnknown);
impl MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<MediaCaptureDeviceExclusiveControlStatus> {
        let this = self;
        unsafe {
            let mut result__: MediaCaptureDeviceExclusiveControlStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureDeviceExclusiveControlStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs;{9d2f920d-a588-43c6-89d6-5ad322af006a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    type Vtable = IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCaptureDeviceExclusiveControlStatusChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureDeviceExclusiveControlStatusChangedEventArgs";
}
impl ::core::convert::From<MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureDeviceExclusiveControlStatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureDeviceExclusiveControlStatusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaCaptureDeviceExclusiveControlStatusChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureFailedEventArgs(::windows::core::IUnknown);
impl MediaCaptureFailedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Code(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCaptureFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFailedEventArgs;{80fde3f4-54c4-42c0-8d19-cea1a87ca18b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCaptureFailedEventArgs {
    type Vtable = IMediaCaptureFailedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCaptureFailedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCaptureFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFailedEventArgs";
}
impl ::core::convert::From<MediaCaptureFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureFailedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureFailedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureFailedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureFailedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureFailedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureFailedEventHandler(pub ::windows::core::IUnknown);
impl MediaCaptureFailedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<MediaCapture>, &::core::option::Option<MediaCaptureFailedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MediaCaptureFailedEventHandlerBox::<F> { vtable: &MediaCaptureFailedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, MediaCapture>, Param1: ::windows::core::IntoParam<'a, MediaCaptureFailedEventArgs>>(&self, sender: Param0, erroreventargs: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), erroreventargs.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct MediaCaptureFailedEventHandlerBox<F: FnMut(&::core::option::Option<MediaCapture>, &::core::option::Option<MediaCaptureFailedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MediaCaptureFailedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<MediaCapture>, &::core::option::Option<MediaCaptureFailedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MediaCaptureFailedEventHandlerBox<F> {
    const VTABLE: MediaCaptureFailedEventHandler_Vtbl = MediaCaptureFailedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MediaCaptureFailedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, erroreventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&erroreventargs)).into()
    }
}
impl ::core::clone::Clone for MediaCaptureFailedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for MediaCaptureFailedEventHandler {
    type Vtable = MediaCaptureFailedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2014effb_5cd8_4f08_a314_0d360da59f14);
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureFailedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2014effb-5cd8-4f08-a314-0d360da59f14}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct MediaCaptureFailedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, erroreventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureFocusChangedEventArgs(::windows::core::IUnknown);
impl MediaCaptureFocusChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Devices\"`*"]
    #[cfg(feature = "Media_Devices")]
    pub fn FocusState(&self) -> ::windows::core::Result<super::Devices::MediaCaptureFocusState> {
        let this = self;
        unsafe {
            let mut result__: super::Devices::MediaCaptureFocusState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FocusState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Devices::MediaCaptureFocusState>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureFocusChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCaptureFocusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureFocusChangedEventArgs;{81e1bc7f-2277-493e-abee-d3f44ff98c04})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCaptureFocusChangedEventArgs {
    type Vtable = IMediaCaptureFocusChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCaptureFocusChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCaptureFocusChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureFocusChangedEventArgs";
}
impl ::core::convert::From<MediaCaptureFocusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureFocusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureFocusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureFocusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureFocusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureFocusChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureFocusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureFocusChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureFocusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureFocusChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaCaptureFocusChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureInitializationSettings(::windows::core::IUnknown);
impl MediaCaptureInitializationSettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MediaCaptureInitializationSettings, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetAudioDeviceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioDeviceId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetVideoDeviceId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoDeviceId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetStreamingCaptureMode(&self, value: StreamingCaptureMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStreamingCaptureMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StreamingCaptureMode(&self) -> ::windows::core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__: StreamingCaptureMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamingCaptureMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamingCaptureMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetPhotoCaptureSource(&self, value: PhotoCaptureSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPhotoCaptureSource)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PhotoCaptureSource(&self) -> ::windows::core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__: PhotoCaptureSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhotoCaptureSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoCaptureSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetMediaCategory(&self, value: MediaCategory) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMediaCategory)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MediaCategory(&self) -> ::windows::core::Result<MediaCategory> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__: MediaCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaCategory)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCategory>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetAudioProcessing(&self, value: super::AudioProcessing) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioProcessing)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings2>(self)?;
        unsafe {
            let mut result__: super::AudioProcessing = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioProcessing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn SetAudioSource<'a, Param0: ::windows::core::IntoParam<'a, super::Core::IMediaSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAudioSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> ::windows::core::Result<super::Core::IMediaSource> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn SetVideoSource<'a, Param0: ::windows::core::IntoParam<'a, super::Core::IMediaSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> ::windows::core::Result<super::Core::IMediaSource> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoProfile(&self) -> ::windows::core::Result<MediaCaptureVideoProfile> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoProfile)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfile>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetVideoProfile<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureVideoProfile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoProfile)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PreviewMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviewMediaDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetPreviewMediaDescription<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPreviewMediaDescription)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn RecordMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordMediaDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetRecordMediaDescription<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRecordMediaDescription)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PhotoMediaDescription(&self) -> ::windows::core::Result<MediaCaptureVideoProfileMediaDescription> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhotoMediaDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureVideoProfileMediaDescription>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetPhotoMediaDescription<'a, Param0: ::windows::core::IntoParam<'a, MediaCaptureVideoProfileMediaDescription>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPhotoMediaDescription)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SourceGroup(&self) -> ::windows::core::Result<Frames::MediaFrameSourceGroup> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceGroup)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Frames::MediaFrameSourceGroup>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn SetSourceGroup<'a, Param0: ::windows::core::IntoParam<'a, Frames::MediaFrameSourceGroup>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSourceGroup)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SharingMode(&self) -> ::windows::core::Result<MediaCaptureSharingMode> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__: MediaCaptureSharingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SharingMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureSharingMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetSharingMode(&self, value: MediaCaptureSharingMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSharingMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MemoryPreference(&self) -> ::windows::core::Result<MediaCaptureMemoryPreference> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe {
            let mut result__: MediaCaptureMemoryPreference = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MemoryPreference)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCaptureMemoryPreference>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetMemoryPreference(&self, value: MediaCaptureMemoryPreference) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMemoryPreference)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AlwaysPlaySystemShutterSound(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlwaysPlaySystemShutterSound)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn SetAlwaysPlaySystemShutterSound(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAlwaysPlaySystemShutterSound)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn DeviceUriPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceUriPasswordCredential)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetDeviceUriPasswordCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDeviceUriPasswordCredential)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeviceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDeviceUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureInitializationSettings7>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDeviceUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaCaptureInitializationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCaptureInitializationSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureInitializationSettings;{9782ba70-ea65-4900-9356-8ca887726884})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCaptureInitializationSettings {
    type Vtable = IMediaCaptureInitializationSettings_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCaptureInitializationSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCaptureInitializationSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureInitializationSettings";
}
impl ::core::convert::From<MediaCaptureInitializationSettings> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureInitializationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureInitializationSettings> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureInitializationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureInitializationSettings> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureInitializationSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureInitializationSettings> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureInitializationSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureInitializationSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureInitializationSettings {}
unsafe impl ::core::marker::Sync for MediaCaptureInitializationSettings {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaCaptureMemoryPreference {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureMemoryPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureMemoryPreference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureMemoryPreference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureMemoryPreference;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCapturePauseResult(::windows::core::IUnknown);
impl MediaCapturePauseResult {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn LastFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastFrame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecordDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCapturePauseResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCapturePauseResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCapturePauseResult;{aec47ca3-4477-4b04-a06f-2c1c5182fe9d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCapturePauseResult {
    type Vtable = IMediaCapturePauseResult_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCapturePauseResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCapturePauseResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCapturePauseResult";
}
impl ::core::convert::From<MediaCapturePauseResult> for ::windows::core::IUnknown {
    fn from(value: MediaCapturePauseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCapturePauseResult> for ::windows::core::IUnknown {
    fn from(value: &MediaCapturePauseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCapturePauseResult> for ::windows::core::IInspectable {
    fn from(value: MediaCapturePauseResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCapturePauseResult> for ::windows::core::IInspectable {
    fn from(value: &MediaCapturePauseResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaCapturePauseResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaCapturePauseResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaCapturePauseResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaCapturePauseResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCapturePauseResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureRelativePanelWatcher(::windows::core::IUnknown);
impl MediaCaptureRelativePanelWatcher {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn RelativePanel(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::Panel> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Enumeration::Panel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativePanel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::Panel>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MediaCaptureRelativePanelWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Changed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaCaptureRelativePanelWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCaptureRelativePanelWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureRelativePanelWatcher;{7d896566-04be-5b89-b30e-bd34a9f12db0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCaptureRelativePanelWatcher {
    type Vtable = IMediaCaptureRelativePanelWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCaptureRelativePanelWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCaptureRelativePanelWatcher {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureRelativePanelWatcher";
}
impl ::core::convert::From<MediaCaptureRelativePanelWatcher> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureRelativePanelWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureRelativePanelWatcher> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureRelativePanelWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureRelativePanelWatcher> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureRelativePanelWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureRelativePanelWatcher> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureRelativePanelWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaCaptureRelativePanelWatcher> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaCaptureRelativePanelWatcher) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaCaptureRelativePanelWatcher> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaCaptureRelativePanelWatcher) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCaptureRelativePanelWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MediaCaptureRelativePanelWatcher {}
unsafe impl ::core::marker::Sync for MediaCaptureRelativePanelWatcher {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureSettings(::windows::core::IUnknown);
impl MediaCaptureSettings {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn StreamingCaptureMode(&self) -> ::windows::core::Result<StreamingCaptureMode> {
        let this = self;
        unsafe {
            let mut result__: StreamingCaptureMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamingCaptureMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StreamingCaptureMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn PhotoCaptureSource(&self) -> ::windows::core::Result<PhotoCaptureSource> {
        let this = self;
        unsafe {
            let mut result__: PhotoCaptureSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhotoCaptureSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PhotoCaptureSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoDeviceCharacteristic(&self) -> ::windows::core::Result<VideoDeviceCharacteristic> {
        let this = self;
        unsafe {
            let mut result__: VideoDeviceCharacteristic = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceCharacteristic)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VideoDeviceCharacteristic>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ConcurrentRecordAndPhotoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConcurrentRecordAndPhotoSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn ConcurrentRecordAndPhotoSequenceSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConcurrentRecordAndPhotoSequenceSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn CameraSoundRequiredForRegion(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CameraSoundRequiredForRegion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Horizontal35mmEquivalentFocalLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Horizontal35mmEquivalentFocalLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PitchOffsetDegrees(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PitchOffsetDegrees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Vertical35mmEquivalentFocalLength(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Vertical35mmEquivalentFocalLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn MediaCategory(&self) -> ::windows::core::Result<MediaCategory> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: MediaCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaCategory)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MediaCategory>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn AudioProcessing(&self) -> ::windows::core::Result<super::AudioProcessing> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings2>(self)?;
        unsafe {
            let mut result__: super::AudioProcessing = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioProcessing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AudioProcessing>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Device(&self) -> ::windows::core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureSettings3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Direct3D11Device)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Graphics::DirectX::Direct3D11::IDirect3DDevice>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCaptureSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureSettings;{1d83aafe-6d45-4477-8dc4-ac5bc01c4091})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCaptureSettings {
    type Vtable = IMediaCaptureSettings_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCaptureSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCaptureSettings {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureSettings";
}
impl ::core::convert::From<MediaCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureSettings> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureSettings> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaCaptureSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureStopResult(::windows::core::IUnknown);
impl MediaCaptureStopResult {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn LastFrame(&self) -> ::windows::core::Result<super::VideoFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastFrame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::VideoFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecordDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecordDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureStopResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCaptureStopResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureStopResult;{f9db6a2a-a092-4ad1-97d4-f201f9d082db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCaptureStopResult {
    type Vtable = IMediaCaptureStopResult_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCaptureStopResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCaptureStopResult {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureStopResult";
}
impl ::core::convert::From<MediaCaptureStopResult> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureStopResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureStopResult> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureStopResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureStopResult> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureStopResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureStopResult> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureStopResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaCaptureStopResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaCaptureStopResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaCaptureStopResult> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaCaptureStopResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MediaCaptureStopResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaCaptureThermalStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCaptureThermalStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCaptureThermalStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCaptureThermalStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCaptureThermalStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureVideoProfile(::windows::core::IUnknown);
impl MediaCaptureVideoProfile {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPreviewMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedPreviewMediaDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedRecordMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedRecordMediaDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPhotoMediaDescription(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedPhotoMediaDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfileMediaDescription>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConcurrency(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConcurrency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<MediaCaptureVideoProfile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`, `\"Media_Capture_Frames\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Capture_Frames"))]
    pub fn FrameSourceInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameSourceInfos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Frames::MediaFrameSourceInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoProfile2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureVideoProfile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCaptureVideoProfile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfile;{21a073bf-a3ee-4ecf-9ef6-50b0bc4e1305})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCaptureVideoProfile {
    type Vtable = IMediaCaptureVideoProfile_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCaptureVideoProfile as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCaptureVideoProfile {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfile";
}
impl ::core::convert::From<MediaCaptureVideoProfile> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfile> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureVideoProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureVideoProfile> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureVideoProfile) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfile> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureVideoProfile) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureVideoProfile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureVideoProfile {}
unsafe impl ::core::marker::Sync for MediaCaptureVideoProfile {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct MediaCaptureVideoProfileMediaDescription(::windows::core::IUnknown);
impl MediaCaptureVideoProfileMediaDescription {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Width(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Width)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn FrameRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsVariablePhotoSequenceSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVariablePhotoSequenceSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsHdrVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHdrVideoSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subtype)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IMediaCaptureVideoProfileMediaDescription2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::clone::Clone for MediaCaptureVideoProfileMediaDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for MediaCaptureVideoProfileMediaDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription;{8012afef-b691-49ff-83f2-c1e76eaaea1b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MediaCaptureVideoProfileMediaDescription {
    type Vtable = IMediaCaptureVideoProfileMediaDescription_Vtbl;
    const IID: ::windows::core::GUID = <IMediaCaptureVideoProfileMediaDescription as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaCaptureVideoProfileMediaDescription {
    const NAME: &'static str = "Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription";
}
impl ::core::convert::From<MediaCaptureVideoProfileMediaDescription> for ::windows::core::IUnknown {
    fn from(value: MediaCaptureVideoProfileMediaDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfileMediaDescription> for ::windows::core::IUnknown {
    fn from(value: &MediaCaptureVideoProfileMediaDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MediaCaptureVideoProfileMediaDescription> for ::windows::core::IInspectable {
    fn from(value: MediaCaptureVideoProfileMediaDescription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MediaCaptureVideoProfileMediaDescription> for ::windows::core::IInspectable {
    fn from(value: &MediaCaptureVideoProfileMediaDescription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MediaCaptureVideoProfileMediaDescription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MediaCaptureVideoProfileMediaDescription {}
unsafe impl ::core::marker::Sync for MediaCaptureVideoProfileMediaDescription {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for MediaStreamType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaStreamType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaStreamType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.MediaStreamType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct OptionalReferencePhotoCapturedEventArgs(::windows::core::IUnknown);
impl OptionalReferencePhotoCapturedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Frame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Context(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Context)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::clone::Clone for OptionalReferencePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for OptionalReferencePhotoCapturedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs;{470f88b3-1e6d-4051-9c8b-f1d85af047b7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for OptionalReferencePhotoCapturedEventArgs {
    type Vtable = IOptionalReferencePhotoCapturedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IOptionalReferencePhotoCapturedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for OptionalReferencePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.OptionalReferencePhotoCapturedEventArgs";
}
impl ::core::convert::From<OptionalReferencePhotoCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: OptionalReferencePhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OptionalReferencePhotoCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &OptionalReferencePhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<OptionalReferencePhotoCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: OptionalReferencePhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&OptionalReferencePhotoCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &OptionalReferencePhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OptionalReferencePhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for OptionalReferencePhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for OptionalReferencePhotoCapturedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PhotoCaptureSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoCaptureSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoCaptureSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoCaptureSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PhotoCaptureSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct PhotoCapturedEventArgs(::windows::core::IUnknown);
impl PhotoCapturedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Frame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Thumbnail(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureTimeOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PhotoCapturedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoCapturedEventArgs;{373bfbc1-984e-4ff0-bf85-1c00aabc5a45})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoCapturedEventArgs {
    type Vtable = IPhotoCapturedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoCapturedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoCapturedEventArgs";
}
impl ::core::convert::From<PhotoCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhotoCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhotoCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoCapturedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct PhotoConfirmationCapturedEventArgs(::windows::core::IUnknown);
impl PhotoConfirmationCapturedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Frame(&self) -> ::windows::core::Result<CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Frame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CapturedFrame>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureTimeOffset)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoConfirmationCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PhotoConfirmationCapturedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.PhotoConfirmationCapturedEventArgs;{ab473672-c28a-4827-8f8d-3636d3beb51e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoConfirmationCapturedEventArgs {
    type Vtable = IPhotoConfirmationCapturedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoConfirmationCapturedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoConfirmationCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.PhotoConfirmationCapturedEventArgs";
}
impl ::core::convert::From<PhotoConfirmationCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhotoConfirmationCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoConfirmationCapturedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhotoConfirmationCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PhotoConfirmationCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhotoConfirmationCapturedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoConfirmationCapturedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhotoConfirmationCapturedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhotoConfirmationCapturedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhotoConfirmationCapturedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoConfirmationCapturedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PowerlineFrequency {
    type Abi = Self;
}
impl ::core::fmt::Debug for PowerlineFrequency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PowerlineFrequency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PowerlineFrequency {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.PowerlineFrequency;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct RecordLimitationExceededEventHandler(pub ::windows::core::IUnknown);
impl RecordLimitationExceededEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<MediaCapture>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RecordLimitationExceededEventHandlerBox::<F> { vtable: &RecordLimitationExceededEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, MediaCapture>>(&self, sender: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct RecordLimitationExceededEventHandlerBox<F: FnMut(&::core::option::Option<MediaCapture>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RecordLimitationExceededEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<MediaCapture>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> RecordLimitationExceededEventHandlerBox<F> {
    const VTABLE: RecordLimitationExceededEventHandler_Vtbl = RecordLimitationExceededEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<RecordLimitationExceededEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
impl ::core::clone::Clone for RecordLimitationExceededEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::Interface for RecordLimitationExceededEventHandler {
    type Vtable = RecordLimitationExceededEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fae8f2e_4fe1_4ffd_aaba_e1f1337d4e53);
}
unsafe impl ::windows::core::RuntimeType for RecordLimitationExceededEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3fae8f2e-4fe1-4ffd-aaba-e1f1337d4e53}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RecordLimitationExceededEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct ScreenCapture(::windows::core::IUnknown);
impl ScreenCapture {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn AudioSource(&self) -> ::windows::core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AudioSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn VideoSource(&self) -> ::windows::core::Result<super::Core::IMediaSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::IMediaSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsAudioSuspended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAudioSuspended)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsVideoSuspended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVideoSuspended)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceSuspensionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ScreenCapture, SourceSuspensionChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceSuspensionChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceSuspensionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSourceSuspensionChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<ScreenCapture> {
        Self::IScreenCaptureStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ScreenCapture>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IScreenCaptureStatics<R, F: FnOnce(&IScreenCaptureStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ScreenCapture, IScreenCaptureStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ScreenCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ScreenCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.ScreenCapture;{89179ef7-cd12-4e0e-a6d4-5b3de98b2e9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ScreenCapture {
    type Vtable = IScreenCapture_Vtbl;
    const IID: ::windows::core::GUID = <IScreenCapture as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ScreenCapture {
    const NAME: &'static str = "Windows.Media.Capture.ScreenCapture";
}
impl ::core::convert::From<ScreenCapture> for ::windows::core::IUnknown {
    fn from(value: ScreenCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenCapture> for ::windows::core::IUnknown {
    fn from(value: &ScreenCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ScreenCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ScreenCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ScreenCapture> for ::windows::core::IInspectable {
    fn from(value: ScreenCapture) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ScreenCapture> for ::windows::core::IInspectable {
    fn from(value: &ScreenCapture) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ScreenCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ScreenCapture {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ScreenCapture {}
unsafe impl ::core::marker::Sync for ScreenCapture {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct SourceSuspensionChangedEventArgs(::windows::core::IUnknown);
impl SourceSuspensionChangedEventArgs {
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsAudioSuspended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAudioSuspended)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    pub fn IsVideoSuspended(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVideoSuspended)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SourceSuspensionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SourceSuspensionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.SourceSuspensionChangedEventArgs;{2ece7b5e-d49b-4394-bc32-f97d6cedec1c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SourceSuspensionChangedEventArgs {
    type Vtable = ISourceSuspensionChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ISourceSuspensionChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SourceSuspensionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.SourceSuspensionChangedEventArgs";
}
impl ::core::convert::From<SourceSuspensionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SourceSuspensionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SourceSuspensionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SourceSuspensionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SourceSuspensionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SourceSuspensionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SourceSuspensionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SourceSuspensionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SourceSuspensionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SourceSuspensionChangedEventArgs {}
unsafe impl ::core::marker::Sync for SourceSuspensionChangedEventArgs {}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for StreamingCaptureMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for StreamingCaptureMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StreamingCaptureMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StreamingCaptureMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.StreamingCaptureMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for VideoDeviceCharacteristic {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoDeviceCharacteristic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoDeviceCharacteristic").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoDeviceCharacteristic {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoDeviceCharacteristic;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for VideoRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for VideoRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VideoRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Capture.VideoRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Capture\"`*"]
#[repr(transparent)]
pub struct VideoStreamConfiguration(::windows::core::IUnknown);
impl VideoStreamConfiguration {
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn InputProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InputProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn OutputProperties(&self) -> ::windows::core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutputProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MediaProperties::VideoEncodingProperties>(result__)
        }
    }
}
impl ::core::clone::Clone for VideoStreamConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for VideoStreamConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.VideoStreamConfiguration;{d8770a6f-4390-4b5e-ad3e-0f8af0963490})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VideoStreamConfiguration {
    type Vtable = IVideoStreamConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <IVideoStreamConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VideoStreamConfiguration {
    const NAME: &'static str = "Windows.Media.Capture.VideoStreamConfiguration";
}
impl ::core::convert::From<VideoStreamConfiguration> for ::windows::core::IUnknown {
    fn from(value: VideoStreamConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStreamConfiguration> for ::windows::core::IUnknown {
    fn from(value: &VideoStreamConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VideoStreamConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VideoStreamConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VideoStreamConfiguration> for ::windows::core::IInspectable {
    fn from(value: VideoStreamConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VideoStreamConfiguration> for ::windows::core::IInspectable {
    fn from(value: &VideoStreamConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VideoStreamConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VideoStreamConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VideoStreamConfiguration {}
unsafe impl ::core::marker::Sync for VideoStreamConfiguration {}
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
unsafe impl ::windows::core::Abi for WhiteBalanceGain {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WhiteBalanceGain {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.Capture.WhiteBalanceGain;f8;f8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for WhiteBalanceGain {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WhiteBalanceGain>()) == 0 }
    }
}
impl ::core::cmp::Eq for WhiteBalanceGain {}
impl ::core::default::Default for WhiteBalanceGain {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
