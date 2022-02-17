#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
pub trait IWebViewControl_Impl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSource(&self, source: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanGoBack(&self) -> ::windows::core::Result<bool>;
    fn CanGoForward(&self) -> ::windows::core::Result<bool>;
    fn SetDefaultBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn DefaultBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool>;
    fn Settings(&self) -> ::windows::core::Result<WebViewControlSettings>;
    fn DeferredPermissionRequests(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>>;
    fn GoForward(&self) -> ::windows::core::Result<()>;
    fn GoBack(&self) -> ::windows::core::Result<()>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Navigate(&self, source: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn NavigateToString(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NavigateToLocalStreamUri(&self, source: &::core::option::Option<super::super::Foundation::Uri>, streamresolver: &::core::option::Option<super::IUriToStreamResolver>) -> ::windows::core::Result<()>;
    fn NavigateWithHttpRequestMessage(&self, requestmessage: &::core::option::Option<super::Http::HttpRequestMessage>) -> ::windows::core::Result<()>;
    fn InvokeScriptAsync(&self, scriptname: &::windows::core::HSTRING, arguments: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CapturePreviewToStreamAsync(&self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>>;
    fn BuildLocalStreamUri(&self, contentidentifier: &::windows::core::HSTRING, relativepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<WebViewControlDeferredPermissionRequest>) -> ::windows::core::Result<()>;
    fn NavigationStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContentLoading(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DOMContentLoaded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDOMContentLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameNavigationStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameContentLoading(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameContentLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameDOMContentLoaded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameDOMContentLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameNavigationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScriptNotify(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScriptNotify(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LongRunningScriptDetected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLongRunningScriptDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnsafeContentWarningDisplaying(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsafeContentWarningDisplaying(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnviewableContentIdentified(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnviewableContentIdentified(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PermissionRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePermissionRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnsupportedUriSchemeIdentified(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsupportedUriSchemeIdentified(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NewWindowRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWindowRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContainsFullScreenElementChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContainsFullScreenElementChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WebResourceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveWebResourceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl ::windows::core::RuntimeName for IWebViewControl {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl IWebViewControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>() -> IWebViewControl_Vtbl {
        unsafe extern "system" fn Source<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSource(::core::mem::transmute(&source)).into()
        }
        unsafe extern "system" fn DocumentTitle<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DocumentTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanGoBack<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanGoBack() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanGoForward<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanGoForward() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultBackgroundColor<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultBackgroundColor(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DefaultBackgroundColor<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsFullScreenElement<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContainsFullScreenElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Settings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeferredPermissionRequests<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeferredPermissionRequests() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GoForward<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GoForward().into()
        }
        unsafe extern "system" fn GoBack<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GoBack().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Navigate<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Navigate(::core::mem::transmute(&source)).into()
        }
        unsafe extern "system" fn NavigateToString<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NavigateToString(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn NavigateToLocalStreamUri<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, streamresolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NavigateToLocalStreamUri(::core::mem::transmute(&source), ::core::mem::transmute(&streamresolver)).into()
        }
        unsafe extern "system" fn NavigateWithHttpRequestMessage<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestmessage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NavigateWithHttpRequestMessage(::core::mem::transmute(&requestmessage)).into()
        }
        unsafe extern "system" fn InvokeScriptAsync<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InvokeScriptAsync(::core::mem::transmute(&scriptname), ::core::mem::transmute(&arguments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePreviewToStreamAsync<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CapturePreviewToStreamAsync(::core::mem::transmute(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureSelectedContentToDataPackageAsync<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CaptureSelectedContentToDataPackageAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildLocalStreamUri<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentidentifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, relativepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BuildLocalStreamUri(::core::mem::transmute(&contentidentifier), ::core::mem::transmute(&relativepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferredPermissionRequestById<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDeferredPermissionRequestById(id, ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn NavigationStarting<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NavigationStarting(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationStarting<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveNavigationStarting(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ContentLoading<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContentLoading(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentLoading<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveContentLoading(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn DOMContentLoaded<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DOMContentLoaded(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDOMContentLoaded<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveDOMContentLoaded(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn NavigationCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NavigationCompleted(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveNavigationCompleted(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameNavigationStarting<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FrameNavigationStarting(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationStarting<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFrameNavigationStarting(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameContentLoading<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FrameContentLoading(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameContentLoading<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFrameContentLoading(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameDOMContentLoaded<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FrameDOMContentLoaded(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameDOMContentLoaded<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFrameDOMContentLoaded(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameNavigationCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FrameNavigationCompleted(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFrameNavigationCompleted(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ScriptNotify<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScriptNotify(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScriptNotify<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveScriptNotify(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn LongRunningScriptDetected<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LongRunningScriptDetected(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLongRunningScriptDetected<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveLongRunningScriptDetected(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UnsafeContentWarningDisplaying<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnsafeContentWarningDisplaying(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnsafeContentWarningDisplaying<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveUnsafeContentWarningDisplaying(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UnviewableContentIdentified<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnviewableContentIdentified(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnviewableContentIdentified<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveUnviewableContentIdentified(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn PermissionRequested<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermissionRequested(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePermissionRequested<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePermissionRequested(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UnsupportedUriSchemeIdentified<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UnsupportedUriSchemeIdentified(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnsupportedUriSchemeIdentified<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveUnsupportedUriSchemeIdentified(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn NewWindowRequested<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NewWindowRequested(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNewWindowRequested<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveNewWindowRequested(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ContainsFullScreenElementChanged<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ContainsFullScreenElementChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContainsFullScreenElementChanged<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveContainsFullScreenElementChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn WebResourceRequested<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WebResourceRequested(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWebResourceRequested<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveWebResourceRequested(::core::mem::transmute(&token)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControl, OFFSET>(),
            Source: Source::<Identity, Impl, OFFSET>,
            SetSource: SetSource::<Identity, Impl, OFFSET>,
            DocumentTitle: DocumentTitle::<Identity, Impl, OFFSET>,
            CanGoBack: CanGoBack::<Identity, Impl, OFFSET>,
            CanGoForward: CanGoForward::<Identity, Impl, OFFSET>,
            SetDefaultBackgroundColor: SetDefaultBackgroundColor::<Identity, Impl, OFFSET>,
            DefaultBackgroundColor: DefaultBackgroundColor::<Identity, Impl, OFFSET>,
            ContainsFullScreenElement: ContainsFullScreenElement::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            DeferredPermissionRequests: DeferredPermissionRequests::<Identity, Impl, OFFSET>,
            GoForward: GoForward::<Identity, Impl, OFFSET>,
            GoBack: GoBack::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Navigate: Navigate::<Identity, Impl, OFFSET>,
            NavigateToString: NavigateToString::<Identity, Impl, OFFSET>,
            NavigateToLocalStreamUri: NavigateToLocalStreamUri::<Identity, Impl, OFFSET>,
            NavigateWithHttpRequestMessage: NavigateWithHttpRequestMessage::<Identity, Impl, OFFSET>,
            InvokeScriptAsync: InvokeScriptAsync::<Identity, Impl, OFFSET>,
            CapturePreviewToStreamAsync: CapturePreviewToStreamAsync::<Identity, Impl, OFFSET>,
            CaptureSelectedContentToDataPackageAsync: CaptureSelectedContentToDataPackageAsync::<Identity, Impl, OFFSET>,
            BuildLocalStreamUri: BuildLocalStreamUri::<Identity, Impl, OFFSET>,
            GetDeferredPermissionRequestById: GetDeferredPermissionRequestById::<Identity, Impl, OFFSET>,
            NavigationStarting: NavigationStarting::<Identity, Impl, OFFSET>,
            RemoveNavigationStarting: RemoveNavigationStarting::<Identity, Impl, OFFSET>,
            ContentLoading: ContentLoading::<Identity, Impl, OFFSET>,
            RemoveContentLoading: RemoveContentLoading::<Identity, Impl, OFFSET>,
            DOMContentLoaded: DOMContentLoaded::<Identity, Impl, OFFSET>,
            RemoveDOMContentLoaded: RemoveDOMContentLoaded::<Identity, Impl, OFFSET>,
            NavigationCompleted: NavigationCompleted::<Identity, Impl, OFFSET>,
            RemoveNavigationCompleted: RemoveNavigationCompleted::<Identity, Impl, OFFSET>,
            FrameNavigationStarting: FrameNavigationStarting::<Identity, Impl, OFFSET>,
            RemoveFrameNavigationStarting: RemoveFrameNavigationStarting::<Identity, Impl, OFFSET>,
            FrameContentLoading: FrameContentLoading::<Identity, Impl, OFFSET>,
            RemoveFrameContentLoading: RemoveFrameContentLoading::<Identity, Impl, OFFSET>,
            FrameDOMContentLoaded: FrameDOMContentLoaded::<Identity, Impl, OFFSET>,
            RemoveFrameDOMContentLoaded: RemoveFrameDOMContentLoaded::<Identity, Impl, OFFSET>,
            FrameNavigationCompleted: FrameNavigationCompleted::<Identity, Impl, OFFSET>,
            RemoveFrameNavigationCompleted: RemoveFrameNavigationCompleted::<Identity, Impl, OFFSET>,
            ScriptNotify: ScriptNotify::<Identity, Impl, OFFSET>,
            RemoveScriptNotify: RemoveScriptNotify::<Identity, Impl, OFFSET>,
            LongRunningScriptDetected: LongRunningScriptDetected::<Identity, Impl, OFFSET>,
            RemoveLongRunningScriptDetected: RemoveLongRunningScriptDetected::<Identity, Impl, OFFSET>,
            UnsafeContentWarningDisplaying: UnsafeContentWarningDisplaying::<Identity, Impl, OFFSET>,
            RemoveUnsafeContentWarningDisplaying: RemoveUnsafeContentWarningDisplaying::<Identity, Impl, OFFSET>,
            UnviewableContentIdentified: UnviewableContentIdentified::<Identity, Impl, OFFSET>,
            RemoveUnviewableContentIdentified: RemoveUnviewableContentIdentified::<Identity, Impl, OFFSET>,
            PermissionRequested: PermissionRequested::<Identity, Impl, OFFSET>,
            RemovePermissionRequested: RemovePermissionRequested::<Identity, Impl, OFFSET>,
            UnsupportedUriSchemeIdentified: UnsupportedUriSchemeIdentified::<Identity, Impl, OFFSET>,
            RemoveUnsupportedUriSchemeIdentified: RemoveUnsupportedUriSchemeIdentified::<Identity, Impl, OFFSET>,
            NewWindowRequested: NewWindowRequested::<Identity, Impl, OFFSET>,
            RemoveNewWindowRequested: RemoveNewWindowRequested::<Identity, Impl, OFFSET>,
            ContainsFullScreenElementChanged: ContainsFullScreenElementChanged::<Identity, Impl, OFFSET>,
            RemoveContainsFullScreenElementChanged: RemoveContainsFullScreenElementChanged::<Identity, Impl, OFFSET>,
            WebResourceRequested: WebResourceRequested::<Identity, Impl, OFFSET>,
            RemoveWebResourceRequested: RemoveWebResourceRequested::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControl as ::windows::core::Interface>::IID
    }
}
pub trait IWebViewControl2_Impl: Sized {
    fn AddInitializeScript(&self, script: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebViewControl2 {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl2";
}
impl IWebViewControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl2_Impl, const OFFSET: isize>() -> IWebViewControl2_Vtbl {
        unsafe extern "system" fn AddInitializeScript<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, script: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddInitializeScript(::core::mem::transmute(&script)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControl2, OFFSET>(),
            AddInitializeScript: AddInitializeScript::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControl2 as ::windows::core::Interface>::IID
    }
}
