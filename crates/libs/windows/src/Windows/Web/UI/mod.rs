#[cfg(feature = "Web_UI_Interop")]
pub mod Interop;
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct IWebViewControl(::windows::core::IUnknown);
impl IWebViewControl {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Source(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Source)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSource(&self, source: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source)).ok() }
    }
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DocumentTitle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanGoBack(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanGoBack)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanGoForward(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanGoForward)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetDefaultBackgroundColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDefaultBackgroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefaultBackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContainsFullScreenElement)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Settings(&self) -> ::windows::core::Result<WebViewControlSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Settings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeferredPermissionRequests(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeferredPermissionRequests)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GoForward(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GoForward)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn GoBack(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GoBack)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Refresh(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Refresh)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Navigate(&self, source: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Navigate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source)).ok() }
    }
    pub fn NavigateToString(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NavigateToString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigateToLocalStreamUri<P0, E0>(&self, source: &super::super::Foundation::Uri, streamresolver: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::IUriToStreamResolver>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NavigateToLocalStreamUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source), streamresolver.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn NavigateWithHttpRequestMessage(&self, requestmessage: &super::Http::HttpRequestMessage) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NavigateWithHttpRequestMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(requestmessage)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InvokeScriptAsync<P0, E0>(&self, scriptname: &::windows::core::HSTRING, arguments: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InvokeScriptAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scriptname), arguments.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CapturePreviewToStreamAsync<P0, E0>(&self, stream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CapturePreviewToStreamAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CaptureSelectedContentToDataPackageAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BuildLocalStreamUri(&self, contentidentifier: &::windows::core::HSTRING, relativepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BuildLocalStreamUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentidentifier), ::core::mem::transmute_copy(relativepath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<WebViewControlDeferredPermissionRequest>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetDeferredPermissionRequestById)(::windows::core::Vtable::as_raw(this), id, result as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationStarting(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NavigationStarting)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNavigationStarting)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentLoading(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentLoading)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentLoading(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveContentLoading)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DOMContentLoaded(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DOMContentLoaded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDOMContentLoaded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDOMContentLoaded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NavigationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNavigationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationStarting(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameNavigationStarting)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFrameNavigationStarting)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameContentLoading(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameContentLoading)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameContentLoading(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFrameContentLoading)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameDOMContentLoaded(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameDOMContentLoaded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameDOMContentLoaded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFrameDOMContentLoaded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameNavigationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFrameNavigationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScriptNotify(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScriptNotify)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScriptNotify(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveScriptNotify)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LongRunningScriptDetected(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LongRunningScriptDetected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLongRunningScriptDetected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveLongRunningScriptDetected)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnsafeContentWarningDisplaying(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnsafeContentWarningDisplaying)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsafeContentWarningDisplaying(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUnsafeContentWarningDisplaying)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnviewableContentIdentified(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnviewableContentIdentified)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnviewableContentIdentified(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUnviewableContentIdentified)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PermissionRequested(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PermissionRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePermissionRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePermissionRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnsupportedUriSchemeIdentified(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnsupportedUriSchemeIdentified)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsupportedUriSchemeIdentified(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUnsupportedUriSchemeIdentified)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewWindowRequested(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewWindowRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNewWindowRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNewWindowRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContainsFullScreenElementChanged(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContainsFullScreenElementChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContainsFullScreenElementChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveContainsFullScreenElementChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WebResourceRequested(&self, handler: &super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WebResourceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWebResourceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveWebResourceRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
::windows::core::interface_hierarchy!(IWebViewControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IWebViewControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebViewControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebViewControl {}
impl ::core::fmt::Debug for IWebViewControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebViewControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebViewControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3f921316-bc70-4bda-9136-c94370899fab}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IWebViewControl {
    type Vtable = IWebViewControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f921316_bc70_4bda_9136_c94370899fab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Source: usize,
    #[cfg(feature = "Foundation")]
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSource: usize,
    pub DocumentTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CanGoBack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanGoForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetDefaultBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetDefaultBackgroundColor: usize,
    #[cfg(feature = "UI")]
    pub DefaultBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DefaultBackgroundColor: usize,
    pub ContainsFullScreenElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeferredPermissionRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeferredPermissionRequests: usize,
    pub GoForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GoBack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Navigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Navigate: usize,
    pub NavigateToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NavigateToLocalStreamUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, streamresolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigateToLocalStreamUri: usize,
    #[cfg(feature = "Web_Http")]
    pub NavigateWithHttpRequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestmessage: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    NavigateWithHttpRequestMessage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InvokeScriptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scriptname: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InvokeScriptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CapturePreviewToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CapturePreviewToStreamAsync: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    pub CaptureSelectedContentToDataPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation")))]
    CaptureSelectedContentToDataPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BuildLocalStreamUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentidentifier: *mut ::core::ffi::c_void, relativepath: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BuildLocalStreamUri: usize,
    pub GetDeferredPermissionRequestById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: u32, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub DOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub FrameNavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameNavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub FrameContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub FrameDOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameDOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub FrameNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ScriptNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScriptNotify: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScriptNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScriptNotify: usize,
    #[cfg(feature = "Foundation")]
    pub LongRunningScriptDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LongRunningScriptDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLongRunningScriptDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLongRunningScriptDetected: usize,
    #[cfg(feature = "Foundation")]
    pub UnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnsafeContentWarningDisplaying: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnsafeContentWarningDisplaying: usize,
    #[cfg(feature = "Foundation")]
    pub UnviewableContentIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnviewableContentIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnviewableContentIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnviewableContentIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub PermissionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PermissionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePermissionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePermissionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnsupportedUriSchemeIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnsupportedUriSchemeIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub NewWindowRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewWindowRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNewWindowRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNewWindowRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContainsFullScreenElementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContainsFullScreenElementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub WebResourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WebResourceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveWebResourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveWebResourceRequested: usize,
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct IWebViewControl2(::windows::core::IUnknown);
impl IWebViewControl2 {
    pub fn AddInitializeScript(&self, script: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AddInitializeScript)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(script)).ok() }
    }
}
::windows::core::interface_hierarchy!(IWebViewControl2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IWebViewControl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebViewControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebViewControl2 {}
impl ::core::fmt::Debug for IWebViewControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebViewControl2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebViewControl2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4d3c06f9-c8df-41cc-8bd5-2a947b204503}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IWebViewControl2 {
    type Vtable = IWebViewControl2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d3c06f9_c8df_41cc_8bd5_2a947b204503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControl2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AddInitializeScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, script: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlContentLoadingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlContentLoadingEventArgs {
    type Vtable = IWebViewControlContentLoadingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlContentLoadingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a3fccb2_b9bb_404b_a22b_66dccd1250c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlContentLoadingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlDOMContentLoadedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlDOMContentLoadedEventArgs {
    type Vtable = IWebViewControlDOMContentLoadedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlDOMContentLoadedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe8bc008_9541_4545_9ff2_2df585b29f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlDOMContentLoadedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlDeferredPermissionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlDeferredPermissionRequest {
    type Vtable = IWebViewControlDeferredPermissionRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlDeferredPermissionRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce349e0_d759_445c_9926_8995298f152b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlDeferredPermissionRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub PermissionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionType) -> ::windows::core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Deny: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlLongRunningScriptDetectedEventArgs {
    type Vtable = IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlLongRunningScriptDetectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a6e5bba_98b4_45bc_bbeb_0f69ce49c599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ExecutionTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecutionTime: usize,
    pub StopPageScriptExecution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStopPageScriptExecution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNavigationCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlNavigationCompletedEventArgs {
    type Vtable = IWebViewControlNavigationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlNavigationCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20409918_4a15_4c46_a55d_f79edb0bde8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNavigationCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub IsSuccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub WebErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WebErrorStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNavigationStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlNavigationStartingEventArgs {
    type Vtable = IWebViewControlNavigationStartingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlNavigationStartingEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c9057c5_0a08_41c7_863b_71e3a9549137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNavigationStartingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlNewWindowRequestedEventArgs {
    type Vtable = IWebViewControlNewWindowRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlNewWindowRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df44bbb_a124_46d5_a083_d02cacdff5ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNewWindowRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub Referrer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referrer: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlNewWindowRequestedEventArgs2 {
    type Vtable = IWebViewControlNewWindowRequestedEventArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlNewWindowRequestedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb53c5ca6_2aae_4bfc_92b9_c30e92b48098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNewWindowRequestedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NewWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNewWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlPermissionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlPermissionRequest {
    type Vtable = IWebViewControlPermissionRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlPermissionRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5bc836c_f22f_40e2_95b2_7729f840eb7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlPermissionRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub PermissionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionType) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionState) -> ::windows::core::HRESULT,
    pub Defer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Deny: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlPermissionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlPermissionRequestedEventArgs {
    type Vtable = IWebViewControlPermissionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlPermissionRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27204d51_2488_4cc5_968e_0a771e59c147);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlPermissionRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PermissionRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlScriptNotifyEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlScriptNotifyEventArgs {
    type Vtable = IWebViewControlScriptNotifyEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlScriptNotifyEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x491de57b_6f49_41bb_b591_51b85b817037);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlScriptNotifyEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlSettings {
    type Vtable = IWebViewControlSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9967fbf_5e98_4cfd_8cce_27b0911e3de8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetIsJavaScriptEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsJavaScriptEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsIndexedDBEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsIndexedDBEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsScriptNotifyAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsScriptNotifyAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    type Vtable = IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3b81944_e4fc_43dc_94ca_f980f30bc51d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlUnviewableContentIdentifiedEventArgs {
    type Vtable = IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlUnviewableContentIdentifiedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a9680db_88f2_4e20_b693_b4e2df4aa581);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub Referrer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referrer: usize,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlWebResourceRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlWebResourceRequestedEventArgs {
    type Vtable = IWebViewControlWebResourceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlWebResourceRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44d6524d_55a4_4d8b_891c_931d8e25d42e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlWebResourceRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    #[cfg(feature = "Web_Http")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Request: usize,
    #[cfg(feature = "Web_Http")]
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    SetResponse: usize,
    #[cfg(feature = "Web_Http")]
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Response: usize,
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlContentLoadingEventArgs(::windows::core::IUnknown);
impl WebViewControlContentLoadingEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlContentLoadingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlContentLoadingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlContentLoadingEventArgs {}
impl ::core::fmt::Debug for WebViewControlContentLoadingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlContentLoadingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlContentLoadingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlContentLoadingEventArgs;{9a3fccb2-b9bb-404b-a22b-66dccd1250c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlContentLoadingEventArgs {
    type Vtable = IWebViewControlContentLoadingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlContentLoadingEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlContentLoadingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlContentLoadingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlContentLoadingEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlContentLoadingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlDOMContentLoadedEventArgs(::windows::core::IUnknown);
impl WebViewControlDOMContentLoadedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlDOMContentLoadedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlDOMContentLoadedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlDOMContentLoadedEventArgs {}
impl ::core::fmt::Debug for WebViewControlDOMContentLoadedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlDOMContentLoadedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlDOMContentLoadedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs;{be8bc008-9541-4545-9ff2-2df585b29f7d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlDOMContentLoadedEventArgs {
    type Vtable = IWebViewControlDOMContentLoadedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlDOMContentLoadedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlDOMContentLoadedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlDOMContentLoadedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlDOMContentLoadedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlDeferredPermissionRequest(::windows::core::IUnknown);
impl WebViewControlDeferredPermissionRequest {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PermissionType(&self) -> ::windows::core::Result<WebViewControlPermissionType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PermissionType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Allow(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Allow)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Deny(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Deny)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for WebViewControlDeferredPermissionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlDeferredPermissionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlDeferredPermissionRequest {}
impl ::core::fmt::Debug for WebViewControlDeferredPermissionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlDeferredPermissionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlDeferredPermissionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlDeferredPermissionRequest;{2ce349e0-d759-445c-9926-8995298f152b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlDeferredPermissionRequest {
    type Vtable = IWebViewControlDeferredPermissionRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlDeferredPermissionRequest {
    const IID: ::windows::core::GUID = <IWebViewControlDeferredPermissionRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlDeferredPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlDeferredPermissionRequest";
}
::windows::core::interface_hierarchy!(WebViewControlDeferredPermissionRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlLongRunningScriptDetectedEventArgs(::windows::core::IUnknown);
impl WebViewControlLongRunningScriptDetectedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecutionTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExecutionTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StopPageScriptExecution(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StopPageScriptExecution)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStopPageScriptExecution(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStopPageScriptExecution)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for WebViewControlLongRunningScriptDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlLongRunningScriptDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlLongRunningScriptDetectedEventArgs {}
impl ::core::fmt::Debug for WebViewControlLongRunningScriptDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlLongRunningScriptDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlLongRunningScriptDetectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs;{2a6e5bba-98b4-45bc-bbeb-0f69ce49c599})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlLongRunningScriptDetectedEventArgs {
    type Vtable = IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlLongRunningScriptDetectedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlLongRunningScriptDetectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlLongRunningScriptDetectedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlLongRunningScriptDetectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlNavigationCompletedEventArgs(::windows::core::IUnknown);
impl WebViewControlNavigationCompletedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSuccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSuccess)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn WebErrorStatus(&self) -> ::windows::core::Result<super::WebErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WebErrorStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlNavigationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlNavigationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlNavigationCompletedEventArgs {}
impl ::core::fmt::Debug for WebViewControlNavigationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlNavigationCompletedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlNavigationCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlNavigationCompletedEventArgs;{20409918-4a15-4c46-a55d-f79edb0bde8b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlNavigationCompletedEventArgs {
    type Vtable = IWebViewControlNavigationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlNavigationCompletedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlNavigationCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNavigationCompletedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlNavigationCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlNavigationStartingEventArgs(::windows::core::IUnknown);
impl WebViewControlNavigationStartingEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCancel)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for WebViewControlNavigationStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlNavigationStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlNavigationStartingEventArgs {}
impl ::core::fmt::Debug for WebViewControlNavigationStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlNavigationStartingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlNavigationStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlNavigationStartingEventArgs;{0c9057c5-0a08-41c7-863b-71e3a9549137})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlNavigationStartingEventArgs {
    type Vtable = IWebViewControlNavigationStartingEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlNavigationStartingEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlNavigationStartingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlNavigationStartingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNavigationStartingEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlNavigationStartingEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlNewWindowRequestedEventArgs(::windows::core::IUnknown);
impl WebViewControlNewWindowRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Referrer(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Referrer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn NewWindow(&self) -> ::windows::core::Result<IWebViewControl> {
        let this = &::windows::core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewWindow)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNewWindow<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IWebViewControl>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNewWindow)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = &::windows::core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlNewWindowRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlNewWindowRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlNewWindowRequestedEventArgs {}
impl ::core::fmt::Debug for WebViewControlNewWindowRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlNewWindowRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlNewWindowRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs;{3df44bbb-a124-46d5-a083-d02cacdff5ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlNewWindowRequestedEventArgs {
    type Vtable = IWebViewControlNewWindowRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlNewWindowRequestedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlNewWindowRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlNewWindowRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlNewWindowRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlPermissionRequest(::windows::core::IUnknown);
impl WebViewControlPermissionRequest {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PermissionType(&self) -> ::windows::core::Result<WebViewControlPermissionType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PermissionType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<WebViewControlPermissionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Defer(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Defer)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Allow(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Allow)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Deny(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Deny)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for WebViewControlPermissionRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlPermissionRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlPermissionRequest {}
impl ::core::fmt::Debug for WebViewControlPermissionRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlPermissionRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlPermissionRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlPermissionRequest;{e5bc836c-f22f-40e2-95b2-7729f840eb7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlPermissionRequest {
    type Vtable = IWebViewControlPermissionRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlPermissionRequest {
    const IID: ::windows::core::GUID = <IWebViewControlPermissionRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlPermissionRequest";
}
::windows::core::interface_hierarchy!(WebViewControlPermissionRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlPermissionRequestedEventArgs(::windows::core::IUnknown);
impl WebViewControlPermissionRequestedEventArgs {
    pub fn PermissionRequest(&self) -> ::windows::core::Result<WebViewControlPermissionRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PermissionRequest)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlPermissionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlPermissionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlPermissionRequestedEventArgs {}
impl ::core::fmt::Debug for WebViewControlPermissionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlPermissionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlPermissionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlPermissionRequestedEventArgs;{27204d51-2488-4cc5-968e-0a771e59c147})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlPermissionRequestedEventArgs {
    type Vtable = IWebViewControlPermissionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlPermissionRequestedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlPermissionRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlPermissionRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlPermissionRequestedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlPermissionRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlScriptNotifyEventArgs(::windows::core::IUnknown);
impl WebViewControlScriptNotifyEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlScriptNotifyEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlScriptNotifyEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlScriptNotifyEventArgs {}
impl ::core::fmt::Debug for WebViewControlScriptNotifyEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlScriptNotifyEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlScriptNotifyEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlScriptNotifyEventArgs;{491de57b-6f49-41bb-b591-51b85b817037})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlScriptNotifyEventArgs {
    type Vtable = IWebViewControlScriptNotifyEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlScriptNotifyEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlScriptNotifyEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlScriptNotifyEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlScriptNotifyEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlScriptNotifyEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlSettings(::windows::core::IUnknown);
impl WebViewControlSettings {
    pub fn SetIsJavaScriptEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsJavaScriptEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsJavaScriptEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsJavaScriptEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsIndexedDBEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsIndexedDBEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsIndexedDBEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsIndexedDBEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsScriptNotifyAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsScriptNotifyAllowed)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsScriptNotifyAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsScriptNotifyAllowed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlSettings {}
impl ::core::fmt::Debug for WebViewControlSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlSettings;{c9967fbf-5e98-4cfd-8cce-27b0911e3de8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlSettings {
    type Vtable = IWebViewControlSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlSettings {
    const IID: ::windows::core::GUID = <IWebViewControlSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlSettings {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlSettings";
}
::windows::core::interface_hierarchy!(WebViewControlSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlUnsupportedUriSchemeIdentifiedEventArgs(::windows::core::IUnknown);
impl WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {}
impl ::core::fmt::Debug for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlUnsupportedUriSchemeIdentifiedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs;{e3b81944-e4fc-43dc-94ca-f980f30bc51d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    type Vtable = IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlUnsupportedUriSchemeIdentifiedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlUnviewableContentIdentifiedEventArgs(::windows::core::IUnknown);
impl WebViewControlUnviewableContentIdentifiedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Referrer(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Referrer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlUnviewableContentIdentifiedEventArgs {}
impl ::core::fmt::Debug for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlUnviewableContentIdentifiedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlUnviewableContentIdentifiedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs;{4a9680db-88f2-4e20-b693-b4e2df4aa581})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlUnviewableContentIdentifiedEventArgs {
    type Vtable = IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlUnviewableContentIdentifiedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlUnviewableContentIdentifiedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlUnviewableContentIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlUnviewableContentIdentifiedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlWebResourceRequestedEventArgs(::windows::core::IUnknown);
impl WebViewControlWebResourceRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn Request(&self) -> ::windows::core::Result<super::Http::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn SetResponse(&self, value: &super::Http::HttpResponseMessage) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetResponse)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn Response(&self) -> ::windows::core::Result<super::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Response)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlWebResourceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlWebResourceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlWebResourceRequestedEventArgs {}
impl ::core::fmt::Debug for WebViewControlWebResourceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlWebResourceRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlWebResourceRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs;{44d6524d-55a4-4d8b-891c-931d8e25d42e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlWebResourceRequestedEventArgs {
    type Vtable = IWebViewControlWebResourceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlWebResourceRequestedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlWebResourceRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlWebResourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlWebResourceRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebViewControlPermissionState(pub i32);
impl WebViewControlPermissionState {
    pub const Unknown: Self = Self(0i32);
    pub const Defer: Self = Self(1i32);
    pub const Allow: Self = Self(2i32);
    pub const Deny: Self = Self(3i32);
}
impl ::core::marker::Copy for WebViewControlPermissionState {}
impl ::core::clone::Clone for WebViewControlPermissionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlPermissionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebViewControlPermissionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebViewControlPermissionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlPermissionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlPermissionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.UI.WebViewControlPermissionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebViewControlPermissionType(pub i32);
impl WebViewControlPermissionType {
    pub const Geolocation: Self = Self(0i32);
    pub const UnlimitedIndexedDBQuota: Self = Self(1i32);
    pub const Media: Self = Self(2i32);
    pub const PointerLock: Self = Self(3i32);
    pub const WebNotifications: Self = Self(4i32);
    pub const Screen: Self = Self(5i32);
    pub const ImmersiveView: Self = Self(6i32);
}
impl ::core::marker::Copy for WebViewControlPermissionType {}
impl ::core::clone::Clone for WebViewControlPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlPermissionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebViewControlPermissionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebViewControlPermissionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlPermissionType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlPermissionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.UI.WebViewControlPermissionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
