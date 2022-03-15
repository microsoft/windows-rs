#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Web_UI_Interop")]
pub mod Interop;
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct IWebViewControl(::windows::core::IUnknown);
impl IWebViewControl {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Source(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentTitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn CanGoBack(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanGoBack)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn CanGoForward(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanGoForward)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetDefaultBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultBackgroundColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultBackgroundColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContainsFullScreenElement)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Settings(&self) -> ::windows::core::Result<WebViewControlSettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Settings)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebViewControlSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeferredPermissionRequests(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeferredPermissionRequests)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn GoForward(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GoForward)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn GoBack(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GoBack)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Refresh(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Refresh)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Navigate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Navigate)(::core::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn NavigateToString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NavigateToString)(::core::mem::transmute_copy(this), text.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigateToLocalStreamUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::IUriToStreamResolver>>(&self, source: Param0, streamresolver: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NavigateToLocalStreamUri)(::core::mem::transmute_copy(this), source.into_param().abi(), streamresolver.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn NavigateWithHttpRequestMessage<'a, Param0: ::windows::core::IntoParam<'a, super::Http::HttpRequestMessage>>(&self, requestmessage: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NavigateWithHttpRequestMessage)(::core::mem::transmute_copy(this), requestmessage.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InvokeScriptAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, scriptname: Param0, arguments: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InvokeScriptAsync)(::core::mem::transmute_copy(this), scriptname.into_param().abi(), arguments.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CapturePreviewToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, stream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CapturePreviewToStreamAsync)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"ApplicationModel_DataTransfer\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CaptureSelectedContentToDataPackageAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BuildLocalStreamUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, contentidentifier: Param0, relativepath: Param1) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BuildLocalStreamUri)(::core::mem::transmute_copy(this), contentidentifier.into_param().abi(), relativepath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<WebViewControlDeferredPermissionRequest>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetDeferredPermissionRequestById)(::core::mem::transmute_copy(this), id, result as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigationStarting)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNavigationStarting)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentLoading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentLoading)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentLoading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContentLoading)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DOMContentLoaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DOMContentLoaded)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDOMContentLoaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDOMContentLoaded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NavigationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNavigationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameNavigationStarting)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameNavigationStarting)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameContentLoading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameContentLoading)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameContentLoading<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameContentLoading)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameDOMContentLoaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameDOMContentLoaded)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameDOMContentLoaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameDOMContentLoaded)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FrameNavigationCompleted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameNavigationCompleted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScriptNotify<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScriptNotify)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScriptNotify<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveScriptNotify)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LongRunningScriptDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LongRunningScriptDetected)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLongRunningScriptDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLongRunningScriptDetected)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnsafeContentWarningDisplaying<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnsafeContentWarningDisplaying)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsafeContentWarningDisplaying<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnsafeContentWarningDisplaying)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnviewableContentIdentified<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnviewableContentIdentified)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnviewableContentIdentified<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnviewableContentIdentified)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PermissionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PermissionRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePermissionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePermissionRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnsupportedUriSchemeIdentified<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnsupportedUriSchemeIdentified)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsupportedUriSchemeIdentified<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnsupportedUriSchemeIdentified)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewWindowRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewWindowRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNewWindowRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNewWindowRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContainsFullScreenElementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContainsFullScreenElementChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContainsFullScreenElementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WebResourceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebResourceRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWebResourceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveWebResourceRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IWebViewControl> for ::windows::core::IUnknown {
    fn from(value: IWebViewControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebViewControl> for ::windows::core::IUnknown {
    fn from(value: &IWebViewControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebViewControl> for ::windows::core::IInspectable {
    fn from(value: IWebViewControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebViewControl> for ::windows::core::IInspectable {
    fn from(value: &IWebViewControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWebViewControl {
    type Vtable = IWebViewControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f921316_bc70_4bda_9136_c94370899fab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControl_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Source: usize,
    #[cfg(feature = "Foundation")]
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSource: usize,
    pub DocumentTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
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
    pub Settings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeferredPermissionRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeferredPermissionRequests: usize,
    pub GoForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GoBack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Navigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Navigate: usize,
    pub NavigateToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NavigateToLocalStreamUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, streamresolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigateToLocalStreamUri: usize,
    #[cfg(feature = "Web_Http")]
    pub NavigateWithHttpRequestMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestmessage: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    NavigateWithHttpRequestMessage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InvokeScriptAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scriptname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InvokeScriptAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CapturePreviewToStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CapturePreviewToStreamAsync: usize,
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    pub CaptureSelectedContentToDataPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation")))]
    CaptureSelectedContentToDataPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BuildLocalStreamUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentidentifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, relativepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BuildLocalStreamUri: usize,
    pub GetDeferredPermissionRequestById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: u32, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub ContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub DOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub NavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub FrameNavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameNavigationStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameNavigationStarting: usize,
    #[cfg(feature = "Foundation")]
    pub FrameContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameContentLoading: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameContentLoading: usize,
    #[cfg(feature = "Foundation")]
    pub FrameDOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameDOMContentLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameDOMContentLoaded: usize,
    #[cfg(feature = "Foundation")]
    pub FrameNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FrameNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFrameNavigationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFrameNavigationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ScriptNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScriptNotify: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScriptNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScriptNotify: usize,
    #[cfg(feature = "Foundation")]
    pub LongRunningScriptDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LongRunningScriptDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLongRunningScriptDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLongRunningScriptDetected: usize,
    #[cfg(feature = "Foundation")]
    pub UnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnsafeContentWarningDisplaying: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnsafeContentWarningDisplaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnsafeContentWarningDisplaying: usize,
    #[cfg(feature = "Foundation")]
    pub UnviewableContentIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnviewableContentIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnviewableContentIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnviewableContentIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub PermissionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PermissionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePermissionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePermissionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnsupportedUriSchemeIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUnsupportedUriSchemeIdentified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUnsupportedUriSchemeIdentified: usize,
    #[cfg(feature = "Foundation")]
    pub NewWindowRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewWindowRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNewWindowRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNewWindowRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContainsFullScreenElementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContainsFullScreenElementChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContainsFullScreenElementChanged: usize,
    #[cfg(feature = "Foundation")]
    pub WebResourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn AddInitializeScript<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, script: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddInitializeScript)(::core::mem::transmute_copy(this), script.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IWebViewControl2> for ::windows::core::IUnknown {
    fn from(value: IWebViewControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebViewControl2> for ::windows::core::IUnknown {
    fn from(value: &IWebViewControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebViewControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebViewControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebViewControl2> for ::windows::core::IInspectable {
    fn from(value: IWebViewControl2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebViewControl2> for ::windows::core::IInspectable {
    fn from(value: &IWebViewControl2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebViewControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebViewControl2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWebViewControl2 {
    type Vtable = IWebViewControl2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d3c06f9_c8df_41cc_8bd5_2a947b204503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControl2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AddInitializeScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, script: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlContentLoadingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlContentLoadingEventArgs {
    type Vtable = IWebViewControlContentLoadingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a3fccb2_b9bb_404b_a22b_66dccd1250c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlContentLoadingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlDOMContentLoadedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlDOMContentLoadedEventArgs {
    type Vtable = IWebViewControlDOMContentLoadedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe8bc008_9541_4545_9ff2_2df585b29f7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlDOMContentLoadedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlDeferredPermissionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlDeferredPermissionRequest {
    type Vtable = IWebViewControlDeferredPermissionRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce349e0_d759_445c_9926_8995298f152b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlDeferredPermissionRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub PermissionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionType) -> ::windows::core::HRESULT,
    pub Allow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Deny: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlLongRunningScriptDetectedEventArgs {
    type Vtable = IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a6e5bba_98b4_45bc_bbeb_0f69ce49c599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
unsafe impl ::windows::core::Interface for IWebViewControlNavigationCompletedEventArgs {
    type Vtable = IWebViewControlNavigationCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20409918_4a15_4c46_a55d_f79edb0bde8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNavigationCompletedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub IsSuccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub WebErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WebErrorStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNavigationStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlNavigationStartingEventArgs {
    type Vtable = IWebViewControlNavigationStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c9057c5_0a08_41c7_863b_71e3a9549137);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNavigationStartingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlNewWindowRequestedEventArgs {
    type Vtable = IWebViewControlNewWindowRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df44bbb_a124_46d5_a083_d02cacdff5ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNewWindowRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub Referrer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referrer: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlNewWindowRequestedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlNewWindowRequestedEventArgs2 {
    type Vtable = IWebViewControlNewWindowRequestedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb53c5ca6_2aae_4bfc_92b9_c30e92b48098);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlNewWindowRequestedEventArgs2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub NewWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetNewWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlPermissionRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlPermissionRequest {
    type Vtable = IWebViewControlPermissionRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5bc836c_f22f_40e2_95b2_7729f840eb7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlPermissionRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IWebViewControlPermissionRequestedEventArgs {
    type Vtable = IWebViewControlPermissionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27204d51_2488_4cc5_968e_0a771e59c147);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlPermissionRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PermissionRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlScriptNotifyEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlScriptNotifyEventArgs {
    type Vtable = IWebViewControlScriptNotifyEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x491de57b_6f49_41bb_b591_51b85b817037);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlScriptNotifyEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlSettings {
    type Vtable = IWebViewControlSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9967fbf_5e98_4cfd_8cce_27b0911e3de8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSettings_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
unsafe impl ::windows::core::Interface for IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    type Vtable = IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3b81944_e4fc_43dc_94ca_f980f30bc51d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlUnviewableContentIdentifiedEventArgs {
    type Vtable = IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a9680db_88f2_4e20_b693_b4e2df4aa581);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub Referrer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Referrer: usize,
    pub MediaType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlWebResourceRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlWebResourceRequestedEventArgs {
    type Vtable = IWebViewControlWebResourceRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44d6524d_55a4_4d8b_891c_931d8e25d42e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlWebResourceRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    #[cfg(feature = "Web_Http")]
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Request: usize,
    #[cfg(feature = "Web_Http")]
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    SetResponse: usize,
    #[cfg(feature = "Web_Http")]
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Web_Http"))]
    Response: usize,
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlContentLoadingEventArgs(::windows::core::IUnknown);
impl WebViewControlContentLoadingEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlContentLoadingEventArgs {
    type Vtable = IWebViewControlContentLoadingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlContentLoadingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlContentLoadingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlContentLoadingEventArgs";
}
impl ::core::convert::From<WebViewControlContentLoadingEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlContentLoadingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlContentLoadingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlContentLoadingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlContentLoadingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlContentLoadingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlContentLoadingEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlContentLoadingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlContentLoadingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlContentLoadingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlContentLoadingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlContentLoadingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlDOMContentLoadedEventArgs(::windows::core::IUnknown);
impl WebViewControlDOMContentLoadedEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlDOMContentLoadedEventArgs {
    type Vtable = IWebViewControlDOMContentLoadedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlDOMContentLoadedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlDOMContentLoadedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlDOMContentLoadedEventArgs";
}
impl ::core::convert::From<WebViewControlDOMContentLoadedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlDOMContentLoadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlDOMContentLoadedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlDOMContentLoadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlDOMContentLoadedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlDOMContentLoadedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlDOMContentLoadedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlDOMContentLoadedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlDOMContentLoadedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlDOMContentLoadedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlDOMContentLoadedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlDOMContentLoadedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlDeferredPermissionRequest(::windows::core::IUnknown);
impl WebViewControlDeferredPermissionRequest {
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn PermissionType(&self) -> ::windows::core::Result<WebViewControlPermissionType> {
        let this = self;
        unsafe {
            let mut result__: WebViewControlPermissionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PermissionType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebViewControlPermissionType>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Allow(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Allow)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Deny(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Deny)(::core::mem::transmute_copy(this)).ok() }
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
unsafe impl ::windows::core::Interface for WebViewControlDeferredPermissionRequest {
    type Vtable = IWebViewControlDeferredPermissionRequest_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlDeferredPermissionRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlDeferredPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlDeferredPermissionRequest";
}
impl ::core::convert::From<WebViewControlDeferredPermissionRequest> for ::windows::core::IUnknown {
    fn from(value: WebViewControlDeferredPermissionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlDeferredPermissionRequest> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlDeferredPermissionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlDeferredPermissionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlDeferredPermissionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlDeferredPermissionRequest> for ::windows::core::IInspectable {
    fn from(value: WebViewControlDeferredPermissionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlDeferredPermissionRequest> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlDeferredPermissionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlDeferredPermissionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlDeferredPermissionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlLongRunningScriptDetectedEventArgs(::windows::core::IUnknown);
impl WebViewControlLongRunningScriptDetectedEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecutionTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecutionTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn StopPageScriptExecution(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopPageScriptExecution)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn SetStopPageScriptExecution(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStopPageScriptExecution)(::core::mem::transmute_copy(this), value).ok() }
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
unsafe impl ::windows::core::Interface for WebViewControlLongRunningScriptDetectedEventArgs {
    type Vtable = IWebViewControlLongRunningScriptDetectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlLongRunningScriptDetectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlLongRunningScriptDetectedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlLongRunningScriptDetectedEventArgs";
}
impl ::core::convert::From<WebViewControlLongRunningScriptDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlLongRunningScriptDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlLongRunningScriptDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlLongRunningScriptDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlLongRunningScriptDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlLongRunningScriptDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlLongRunningScriptDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlLongRunningScriptDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlLongRunningScriptDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlLongRunningScriptDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlLongRunningScriptDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlLongRunningScriptDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlNavigationCompletedEventArgs(::windows::core::IUnknown);
impl WebViewControlNavigationCompletedEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn IsSuccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSuccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn WebErrorStatus(&self) -> ::windows::core::Result<super::WebErrorStatus> {
        let this = self;
        unsafe {
            let mut result__: super::WebErrorStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebErrorStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::WebErrorStatus>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlNavigationCompletedEventArgs {
    type Vtable = IWebViewControlNavigationCompletedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlNavigationCompletedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNavigationCompletedEventArgs";
}
impl ::core::convert::From<WebViewControlNavigationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlNavigationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNavigationCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlNavigationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlNavigationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlNavigationCompletedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNavigationCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlNavigationCompletedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlNavigationCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlNavigationStartingEventArgs(::windows::core::IUnknown);
impl WebViewControlNavigationStartingEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCancel)(::core::mem::transmute_copy(this), value).ok() }
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
unsafe impl ::windows::core::Interface for WebViewControlNavigationStartingEventArgs {
    type Vtable = IWebViewControlNavigationStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlNavigationStartingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlNavigationStartingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNavigationStartingEventArgs";
}
impl ::core::convert::From<WebViewControlNavigationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlNavigationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNavigationStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlNavigationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlNavigationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlNavigationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlNavigationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlNavigationStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNavigationStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlNavigationStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlNavigationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlNavigationStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlNewWindowRequestedEventArgs(::windows::core::IUnknown);
impl WebViewControlNewWindowRequestedEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Referrer(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Referrer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn NewWindow(&self) -> ::windows::core::Result<IWebViewControl> {
        let this = &::windows::core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewWindow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IWebViewControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn SetNewWindow<'a, Param0: ::windows::core::IntoParam<'a, IWebViewControl>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetNewWindow)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = &::windows::core::Interface::cast::<IWebViewControlNewWindowRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlNewWindowRequestedEventArgs {
    type Vtable = IWebViewControlNewWindowRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlNewWindowRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlNewWindowRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlNewWindowRequestedEventArgs";
}
impl ::core::convert::From<WebViewControlNewWindowRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlNewWindowRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNewWindowRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlNewWindowRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlNewWindowRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlNewWindowRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlNewWindowRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlNewWindowRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlNewWindowRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlNewWindowRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlNewWindowRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlNewWindowRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlPermissionRequest(::windows::core::IUnknown);
impl WebViewControlPermissionRequest {
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn PermissionType(&self) -> ::windows::core::Result<WebViewControlPermissionType> {
        let this = self;
        unsafe {
            let mut result__: WebViewControlPermissionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PermissionType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebViewControlPermissionType>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn State(&self) -> ::windows::core::Result<WebViewControlPermissionState> {
        let this = self;
        unsafe {
            let mut result__: WebViewControlPermissionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebViewControlPermissionState>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Defer(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Defer)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Allow(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Allow)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Deny(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Deny)(::core::mem::transmute_copy(this)).ok() }
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
unsafe impl ::windows::core::Interface for WebViewControlPermissionRequest {
    type Vtable = IWebViewControlPermissionRequest_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlPermissionRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlPermissionRequest";
}
impl ::core::convert::From<WebViewControlPermissionRequest> for ::windows::core::IUnknown {
    fn from(value: WebViewControlPermissionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlPermissionRequest> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlPermissionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlPermissionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlPermissionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlPermissionRequest> for ::windows::core::IInspectable {
    fn from(value: WebViewControlPermissionRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlPermissionRequest> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlPermissionRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlPermissionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlPermissionRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlPermissionRequestedEventArgs(::windows::core::IUnknown);
impl WebViewControlPermissionRequestedEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn PermissionRequest(&self) -> ::windows::core::Result<WebViewControlPermissionRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PermissionRequest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebViewControlPermissionRequest>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlPermissionRequestedEventArgs {
    type Vtable = IWebViewControlPermissionRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlPermissionRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlPermissionRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlPermissionRequestedEventArgs";
}
impl ::core::convert::From<WebViewControlPermissionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlPermissionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlPermissionRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlPermissionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlPermissionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlPermissionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlPermissionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlPermissionRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlPermissionRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlPermissionRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlPermissionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlPermissionRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlScriptNotifyEventArgs(::windows::core::IUnknown);
impl WebViewControlScriptNotifyEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlScriptNotifyEventArgs {
    type Vtable = IWebViewControlScriptNotifyEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlScriptNotifyEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlScriptNotifyEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlScriptNotifyEventArgs";
}
impl ::core::convert::From<WebViewControlScriptNotifyEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlScriptNotifyEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlScriptNotifyEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlScriptNotifyEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlScriptNotifyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlScriptNotifyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlScriptNotifyEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlScriptNotifyEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlScriptNotifyEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlScriptNotifyEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlScriptNotifyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlScriptNotifyEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlSettings(::windows::core::IUnknown);
impl WebViewControlSettings {
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn SetIsJavaScriptEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsJavaScriptEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn IsJavaScriptEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsJavaScriptEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn SetIsIndexedDBEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsIndexedDBEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn IsIndexedDBEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsIndexedDBEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn SetIsScriptNotifyAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsScriptNotifyAllowed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn IsScriptNotifyAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsScriptNotifyAllowed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlSettings {
    type Vtable = IWebViewControlSettings_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlSettings {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlSettings";
}
impl ::core::convert::From<WebViewControlSettings> for ::windows::core::IUnknown {
    fn from(value: WebViewControlSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlSettings> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlSettings> for ::windows::core::IInspectable {
    fn from(value: WebViewControlSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlSettings> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlUnsupportedUriSchemeIdentifiedEventArgs(::windows::core::IUnknown);
impl WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::core::mem::transmute_copy(this), value).ok() }
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
unsafe impl ::windows::core::Interface for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    type Vtable = IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlUnsupportedUriSchemeIdentifiedEventArgs";
}
impl ::core::convert::From<WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlUnsupportedUriSchemeIdentifiedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlUnsupportedUriSchemeIdentifiedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlUnsupportedUriSchemeIdentifiedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlUnsupportedUriSchemeIdentifiedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlUnviewableContentIdentifiedEventArgs(::windows::core::IUnknown);
impl WebViewControlUnviewableContentIdentifiedEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Referrer(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Referrer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    pub fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MediaType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlUnviewableContentIdentifiedEventArgs {
    type Vtable = IWebViewControlUnviewableContentIdentifiedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlUnviewableContentIdentifiedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlUnviewableContentIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlUnviewableContentIdentifiedEventArgs";
}
impl ::core::convert::From<WebViewControlUnviewableContentIdentifiedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlUnviewableContentIdentifiedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlUnviewableContentIdentifiedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlUnviewableContentIdentifiedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlUnviewableContentIdentifiedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlUnviewableContentIdentifiedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlUnviewableContentIdentifiedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlUnviewableContentIdentifiedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlUnviewableContentIdentifiedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlUnviewableContentIdentifiedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlUnviewableContentIdentifiedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI\"`*"]
#[repr(transparent)]
pub struct WebViewControlWebResourceRequestedEventArgs(::windows::core::IUnknown);
impl WebViewControlWebResourceRequestedEventArgs {
    #[doc = "*Required features: `\"Web_UI\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn Request(&self) -> ::windows::core::Result<super::Http::HttpRequestMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Http::HttpRequestMessage>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn SetResponse<'a, Param0: ::windows::core::IntoParam<'a, super::Http::HttpResponseMessage>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResponse)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`, `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn Response(&self) -> ::windows::core::Result<super::Http::HttpResponseMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Response)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Http::HttpResponseMessage>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlWebResourceRequestedEventArgs {
    type Vtable = IWebViewControlWebResourceRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlWebResourceRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlWebResourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.WebViewControlWebResourceRequestedEventArgs";
}
impl ::core::convert::From<WebViewControlWebResourceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlWebResourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlWebResourceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlWebResourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlWebResourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlWebResourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlWebResourceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlWebResourceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlWebResourceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlWebResourceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlWebResourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlWebResourceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
