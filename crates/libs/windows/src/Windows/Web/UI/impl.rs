#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
pub trait IWebViewControlImpl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSource(&mut self, source: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn DocumentTitle(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanGoBack(&mut self) -> ::windows::core::Result<bool>;
    fn CanGoForward(&mut self) -> ::windows::core::Result<bool>;
    fn SetDefaultBackgroundColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn DefaultBackgroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn ContainsFullScreenElement(&mut self) -> ::windows::core::Result<bool>;
    fn Settings(&mut self) -> ::windows::core::Result<WebViewControlSettings>;
    fn DeferredPermissionRequests(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>>;
    fn GoForward(&mut self) -> ::windows::core::Result<()>;
    fn GoBack(&mut self) -> ::windows::core::Result<()>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Navigate(&mut self, source: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn NavigateToString(&mut self, text: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NavigateToLocalStreamUri(&mut self, source: &::core::option::Option<super::super::Foundation::Uri>, streamresolver: &::core::option::Option<super::IUriToStreamResolver>) -> ::windows::core::Result<()>;
    fn NavigateWithHttpRequestMessage(&mut self, requestmessage: &::core::option::Option<super::Http::HttpRequestMessage>) -> ::windows::core::Result<()>;
    fn InvokeScriptAsync(&mut self, scriptname: &::windows::core::HSTRING, arguments: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CapturePreviewToStreamAsync(&mut self, stream: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CaptureSelectedContentToDataPackageAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>>;
    fn BuildLocalStreamUri(&mut self, contentidentifier: &::windows::core::HSTRING, relativepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetDeferredPermissionRequestById(&mut self, id: u32, result: &mut ::core::option::Option<WebViewControlDeferredPermissionRequest>) -> ::windows::core::Result<()>;
    fn NavigationStarting(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContentLoading(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentLoading(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DOMContentLoaded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDOMContentLoaded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameNavigationStarting(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationStarting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameContentLoading(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameContentLoading(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameDOMContentLoaded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameDOMContentLoaded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FrameNavigationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ScriptNotify(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScriptNotify(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LongRunningScriptDetected(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLongRunningScriptDetected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnsafeContentWarningDisplaying(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsafeContentWarningDisplaying(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnviewableContentIdentified(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnviewableContentIdentified(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PermissionRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePermissionRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UnsupportedUriSchemeIdentified(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsupportedUriSchemeIdentified(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NewWindowRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWindowRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContainsFullScreenElementChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContainsFullScreenElementChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WebResourceRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveWebResourceRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl ::windows::core::RuntimeName for IWebViewControl {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl IWebViewControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlVtbl {
        unsafe extern "system" fn Source<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&source as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DocumentTitle<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DocumentTitle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanGoBack<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanGoBack() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanGoForward<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanGoForward() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultBackgroundColor<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultBackgroundColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultBackgroundColor<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsFullScreenElement<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainsFullScreenElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Settings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeferredPermissionRequests<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeferredPermissionRequests() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GoForward<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GoForward().into()
        }
        unsafe extern "system" fn GoBack<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GoBack().into()
        }
        unsafe extern "system" fn Refresh<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Stop<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Navigate<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Navigate(&*(&source as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigateToString<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NavigateToString(&*(&text as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigateToLocalStreamUri<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: ::windows::core::RawPtr, streamresolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NavigateToLocalStreamUri(&*(&source as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&streamresolver as *const <super::IUriToStreamResolver as ::windows::core::Abi>::Abi as *const <super::IUriToStreamResolver as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigateWithHttpRequestMessage<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestmessage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NavigateWithHttpRequestMessage(&*(&requestmessage as *const <super::Http::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::Http::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InvokeScriptAsync<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvokeScriptAsync(&*(&scriptname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&arguments as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePreviewToStreamAsync<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapturePreviewToStreamAsync(&*(&stream as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IRandomAccessStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureSelectedContentToDataPackageAsync<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaptureSelectedContentToDataPackageAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildLocalStreamUri<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentidentifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, relativepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildLocalStreamUri(&*(&contentidentifier as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&relativepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferredPermissionRequestById<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, result: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeferredPermissionRequestById(id, ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn NavigationStarting<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationStarting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationStarting<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNavigationStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentLoading<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentLoading(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentLoading<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContentLoading(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DOMContentLoaded<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DOMContentLoaded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDOMContentLoaded<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDOMContentLoaded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigationCompleted<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationCompleted<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNavigationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FrameNavigationStarting<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameNavigationStarting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationStarting<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameNavigationStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FrameContentLoading<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameContentLoading(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameContentLoading<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameContentLoading(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FrameDOMContentLoaded<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameDOMContentLoaded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameDOMContentLoaded<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameDOMContentLoaded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FrameNavigationCompleted<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameNavigationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationCompleted<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFrameNavigationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScriptNotify<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScriptNotify(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScriptNotify<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScriptNotify(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LongRunningScriptDetected<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongRunningScriptDetected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLongRunningScriptDetected<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLongRunningScriptDetected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnsafeContentWarningDisplaying<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnsafeContentWarningDisplaying(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnsafeContentWarningDisplaying<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUnsafeContentWarningDisplaying(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnviewableContentIdentified<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnviewableContentIdentified(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnviewableContentIdentified<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUnviewableContentIdentified(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PermissionRequested<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermissionRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePermissionRequested<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePermissionRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnsupportedUriSchemeIdentified<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnsupportedUriSchemeIdentified(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnsupportedUriSchemeIdentified<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUnsupportedUriSchemeIdentified(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NewWindowRequested<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewWindowRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNewWindowRequested<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNewWindowRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContainsFullScreenElementChanged<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainsFullScreenElementChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContainsFullScreenElementChanged<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContainsFullScreenElementChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WebResourceRequested<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebResourceRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWebResourceRequested<Impl: IWebViewControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWebResourceRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControl, BASE_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            DocumentTitle: DocumentTitle::<Impl, IMPL_OFFSET>,
            CanGoBack: CanGoBack::<Impl, IMPL_OFFSET>,
            CanGoForward: CanGoForward::<Impl, IMPL_OFFSET>,
            SetDefaultBackgroundColor: SetDefaultBackgroundColor::<Impl, IMPL_OFFSET>,
            DefaultBackgroundColor: DefaultBackgroundColor::<Impl, IMPL_OFFSET>,
            ContainsFullScreenElement: ContainsFullScreenElement::<Impl, IMPL_OFFSET>,
            Settings: Settings::<Impl, IMPL_OFFSET>,
            DeferredPermissionRequests: DeferredPermissionRequests::<Impl, IMPL_OFFSET>,
            GoForward: GoForward::<Impl, IMPL_OFFSET>,
            GoBack: GoBack::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Navigate: Navigate::<Impl, IMPL_OFFSET>,
            NavigateToString: NavigateToString::<Impl, IMPL_OFFSET>,
            NavigateToLocalStreamUri: NavigateToLocalStreamUri::<Impl, IMPL_OFFSET>,
            NavigateWithHttpRequestMessage: NavigateWithHttpRequestMessage::<Impl, IMPL_OFFSET>,
            InvokeScriptAsync: InvokeScriptAsync::<Impl, IMPL_OFFSET>,
            CapturePreviewToStreamAsync: CapturePreviewToStreamAsync::<Impl, IMPL_OFFSET>,
            CaptureSelectedContentToDataPackageAsync: CaptureSelectedContentToDataPackageAsync::<Impl, IMPL_OFFSET>,
            BuildLocalStreamUri: BuildLocalStreamUri::<Impl, IMPL_OFFSET>,
            GetDeferredPermissionRequestById: GetDeferredPermissionRequestById::<Impl, IMPL_OFFSET>,
            NavigationStarting: NavigationStarting::<Impl, IMPL_OFFSET>,
            RemoveNavigationStarting: RemoveNavigationStarting::<Impl, IMPL_OFFSET>,
            ContentLoading: ContentLoading::<Impl, IMPL_OFFSET>,
            RemoveContentLoading: RemoveContentLoading::<Impl, IMPL_OFFSET>,
            DOMContentLoaded: DOMContentLoaded::<Impl, IMPL_OFFSET>,
            RemoveDOMContentLoaded: RemoveDOMContentLoaded::<Impl, IMPL_OFFSET>,
            NavigationCompleted: NavigationCompleted::<Impl, IMPL_OFFSET>,
            RemoveNavigationCompleted: RemoveNavigationCompleted::<Impl, IMPL_OFFSET>,
            FrameNavigationStarting: FrameNavigationStarting::<Impl, IMPL_OFFSET>,
            RemoveFrameNavigationStarting: RemoveFrameNavigationStarting::<Impl, IMPL_OFFSET>,
            FrameContentLoading: FrameContentLoading::<Impl, IMPL_OFFSET>,
            RemoveFrameContentLoading: RemoveFrameContentLoading::<Impl, IMPL_OFFSET>,
            FrameDOMContentLoaded: FrameDOMContentLoaded::<Impl, IMPL_OFFSET>,
            RemoveFrameDOMContentLoaded: RemoveFrameDOMContentLoaded::<Impl, IMPL_OFFSET>,
            FrameNavigationCompleted: FrameNavigationCompleted::<Impl, IMPL_OFFSET>,
            RemoveFrameNavigationCompleted: RemoveFrameNavigationCompleted::<Impl, IMPL_OFFSET>,
            ScriptNotify: ScriptNotify::<Impl, IMPL_OFFSET>,
            RemoveScriptNotify: RemoveScriptNotify::<Impl, IMPL_OFFSET>,
            LongRunningScriptDetected: LongRunningScriptDetected::<Impl, IMPL_OFFSET>,
            RemoveLongRunningScriptDetected: RemoveLongRunningScriptDetected::<Impl, IMPL_OFFSET>,
            UnsafeContentWarningDisplaying: UnsafeContentWarningDisplaying::<Impl, IMPL_OFFSET>,
            RemoveUnsafeContentWarningDisplaying: RemoveUnsafeContentWarningDisplaying::<Impl, IMPL_OFFSET>,
            UnviewableContentIdentified: UnviewableContentIdentified::<Impl, IMPL_OFFSET>,
            RemoveUnviewableContentIdentified: RemoveUnviewableContentIdentified::<Impl, IMPL_OFFSET>,
            PermissionRequested: PermissionRequested::<Impl, IMPL_OFFSET>,
            RemovePermissionRequested: RemovePermissionRequested::<Impl, IMPL_OFFSET>,
            UnsupportedUriSchemeIdentified: UnsupportedUriSchemeIdentified::<Impl, IMPL_OFFSET>,
            RemoveUnsupportedUriSchemeIdentified: RemoveUnsupportedUriSchemeIdentified::<Impl, IMPL_OFFSET>,
            NewWindowRequested: NewWindowRequested::<Impl, IMPL_OFFSET>,
            RemoveNewWindowRequested: RemoveNewWindowRequested::<Impl, IMPL_OFFSET>,
            ContainsFullScreenElementChanged: ContainsFullScreenElementChanged::<Impl, IMPL_OFFSET>,
            RemoveContainsFullScreenElementChanged: RemoveContainsFullScreenElementChanged::<Impl, IMPL_OFFSET>,
            WebResourceRequested: WebResourceRequested::<Impl, IMPL_OFFSET>,
            RemoveWebResourceRequested: RemoveWebResourceRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControl as ::windows::core::Interface>::IID
    }
}
pub trait IWebViewControl2Impl: Sized {
    fn AddInitializeScript(&mut self, script: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebViewControl2 {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl2";
}
impl IWebViewControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControl2Vtbl {
        unsafe extern "system" fn AddInitializeScript<Impl: IWebViewControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, script: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInitializeScript(&*(&script as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControl2, BASE_OFFSET>(),
            AddInitializeScript: AddInitializeScript::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlContentLoadingEventArgsImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlContentLoadingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlContentLoadingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlContentLoadingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlContentLoadingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlContentLoadingEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IWebViewControlContentLoadingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlContentLoadingEventArgs, BASE_OFFSET>(), Uri: Uri::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlContentLoadingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlDOMContentLoadedEventArgsImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlDOMContentLoadedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlDOMContentLoadedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlDOMContentLoadedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlDOMContentLoadedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlDOMContentLoadedEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IWebViewControlDOMContentLoadedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlDOMContentLoadedEventArgs, BASE_OFFSET>(), Uri: Uri::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlDOMContentLoadedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlDeferredPermissionRequestImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn PermissionType(&mut self) -> ::windows::core::Result<WebViewControlPermissionType>;
    fn Allow(&mut self) -> ::windows::core::Result<()>;
    fn Deny(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlDeferredPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlDeferredPermissionRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlDeferredPermissionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlDeferredPermissionRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlDeferredPermissionRequestVtbl {
        unsafe extern "system" fn Id<Impl: IWebViewControlDeferredPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IWebViewControlDeferredPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermissionType<Impl: IWebViewControlDeferredPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermissionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Allow<Impl: IWebViewControlDeferredPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Allow().into()
        }
        unsafe extern "system" fn Deny<Impl: IWebViewControlDeferredPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deny().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlDeferredPermissionRequest, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            PermissionType: PermissionType::<Impl, IMPL_OFFSET>,
            Allow: Allow::<Impl, IMPL_OFFSET>,
            Deny: Deny::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlDeferredPermissionRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlLongRunningScriptDetectedEventArgsImpl: Sized {
    fn ExecutionTime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn StopPageScriptExecution(&mut self) -> ::windows::core::Result<bool>;
    fn SetStopPageScriptExecution(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlLongRunningScriptDetectedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlLongRunningScriptDetectedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlLongRunningScriptDetectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlLongRunningScriptDetectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlLongRunningScriptDetectedEventArgsVtbl {
        unsafe extern "system" fn ExecutionTime<Impl: IWebViewControlLongRunningScriptDetectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecutionTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopPageScriptExecution<Impl: IWebViewControlLongRunningScriptDetectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopPageScriptExecution() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopPageScriptExecution<Impl: IWebViewControlLongRunningScriptDetectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopPageScriptExecution(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlLongRunningScriptDetectedEventArgs, BASE_OFFSET>(),
            ExecutionTime: ExecutionTime::<Impl, IMPL_OFFSET>,
            StopPageScriptExecution: StopPageScriptExecution::<Impl, IMPL_OFFSET>,
            SetStopPageScriptExecution: SetStopPageScriptExecution::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlLongRunningScriptDetectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlNavigationCompletedEventArgsImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn IsSuccess(&mut self) -> ::windows::core::Result<bool>;
    fn WebErrorStatus(&mut self) -> ::windows::core::Result<super::WebErrorStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlNavigationCompletedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlNavigationCompletedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlNavigationCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlNavigationCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlNavigationCompletedEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IWebViewControlNavigationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSuccess<Impl: IWebViewControlNavigationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSuccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebErrorStatus<Impl: IWebViewControlNavigationCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::WebErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebErrorStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlNavigationCompletedEventArgs, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            IsSuccess: IsSuccess::<Impl, IMPL_OFFSET>,
            WebErrorStatus: WebErrorStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlNavigationCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlNavigationStartingEventArgsImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Cancel(&mut self) -> ::windows::core::Result<bool>;
    fn SetCancel(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlNavigationStartingEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlNavigationStartingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlNavigationStartingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlNavigationStartingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlNavigationStartingEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IWebViewControlNavigationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IWebViewControlNavigationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancel<Impl: IWebViewControlNavigationStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlNavigationStartingEventArgs, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            SetCancel: SetCancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlNavigationStartingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlNewWindowRequestedEventArgsImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Referrer(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlNewWindowRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlNewWindowRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlNewWindowRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlNewWindowRequestedEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IWebViewControlNewWindowRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Referrer<Impl: IWebViewControlNewWindowRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Referrer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IWebViewControlNewWindowRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IWebViewControlNewWindowRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlNewWindowRequestedEventArgs, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Referrer: Referrer::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlNewWindowRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlNewWindowRequestedEventArgs2Impl: Sized {
    fn NewWindow(&mut self) -> ::windows::core::Result<IWebViewControl>;
    fn SetNewWindow(&mut self, value: &::core::option::Option<IWebViewControl>) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlNewWindowRequestedEventArgs2 {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlNewWindowRequestedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlNewWindowRequestedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlNewWindowRequestedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlNewWindowRequestedEventArgs2Vtbl {
        unsafe extern "system" fn NewWindow<Impl: IWebViewControlNewWindowRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNewWindow<Impl: IWebViewControlNewWindowRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNewWindow(&*(&value as *const <IWebViewControl as ::windows::core::Abi>::Abi as *const <IWebViewControl as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IWebViewControlNewWindowRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlNewWindowRequestedEventArgs2, BASE_OFFSET>(),
            NewWindow: NewWindow::<Impl, IMPL_OFFSET>,
            SetNewWindow: SetNewWindow::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlNewWindowRequestedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlPermissionRequestImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn PermissionType(&mut self) -> ::windows::core::Result<WebViewControlPermissionType>;
    fn State(&mut self) -> ::windows::core::Result<WebViewControlPermissionState>;
    fn Defer(&mut self) -> ::windows::core::Result<()>;
    fn Allow(&mut self) -> ::windows::core::Result<()>;
    fn Deny(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlPermissionRequest {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlPermissionRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlPermissionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlPermissionRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlPermissionRequestVtbl {
        unsafe extern "system" fn Id<Impl: IWebViewControlPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: IWebViewControlPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PermissionType<Impl: IWebViewControlPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermissionType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IWebViewControlPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlPermissionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Defer<Impl: IWebViewControlPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Defer().into()
        }
        unsafe extern "system" fn Allow<Impl: IWebViewControlPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Allow().into()
        }
        unsafe extern "system" fn Deny<Impl: IWebViewControlPermissionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deny().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlPermissionRequest, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            PermissionType: PermissionType::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Defer: Defer::<Impl, IMPL_OFFSET>,
            Allow: Allow::<Impl, IMPL_OFFSET>,
            Deny: Deny::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlPermissionRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlPermissionRequestedEventArgsImpl: Sized {
    fn PermissionRequest(&mut self) -> ::windows::core::Result<WebViewControlPermissionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebViewControlPermissionRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlPermissionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebViewControlPermissionRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlPermissionRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlPermissionRequestedEventArgsVtbl {
        unsafe extern "system" fn PermissionRequest<Impl: IWebViewControlPermissionRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermissionRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlPermissionRequestedEventArgs, BASE_OFFSET>(),
            PermissionRequest: PermissionRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlPermissionRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlScriptNotifyEventArgsImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlScriptNotifyEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlScriptNotifyEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlScriptNotifyEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlScriptNotifyEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlScriptNotifyEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IWebViewControlScriptNotifyEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IWebViewControlScriptNotifyEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlScriptNotifyEventArgs, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlScriptNotifyEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebViewControlSettingsImpl: Sized {
    fn SetIsJavaScriptEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsJavaScriptEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsIndexedDBEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsIndexedDBEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsScriptNotifyAllowed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsScriptNotifyAllowed(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebViewControlSettings {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IWebViewControlSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlSettingsVtbl {
        unsafe extern "system" fn SetIsJavaScriptEnabled<Impl: IWebViewControlSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsJavaScriptEnabled(value).into()
        }
        unsafe extern "system" fn IsJavaScriptEnabled<Impl: IWebViewControlSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsJavaScriptEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsIndexedDBEnabled<Impl: IWebViewControlSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsIndexedDBEnabled(value).into()
        }
        unsafe extern "system" fn IsIndexedDBEnabled<Impl: IWebViewControlSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIndexedDBEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsScriptNotifyAllowed<Impl: IWebViewControlSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsScriptNotifyAllowed(value).into()
        }
        unsafe extern "system" fn IsScriptNotifyAllowed<Impl: IWebViewControlSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScriptNotifyAllowed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlSettings, BASE_OFFSET>(),
            SetIsJavaScriptEnabled: SetIsJavaScriptEnabled::<Impl, IMPL_OFFSET>,
            IsJavaScriptEnabled: IsJavaScriptEnabled::<Impl, IMPL_OFFSET>,
            SetIsIndexedDBEnabled: SetIsIndexedDBEnabled::<Impl, IMPL_OFFSET>,
            IsIndexedDBEnabled: IsIndexedDBEnabled::<Impl, IMPL_OFFSET>,
            SetIsScriptNotifyAllowed: SetIsScriptNotifyAllowed::<Impl, IMPL_OFFSET>,
            IsScriptNotifyAllowed: IsScriptNotifyAllowed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlUnsupportedUriSchemeIdentifiedEventArgsImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlUnsupportedUriSchemeIdentifiedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlUnsupportedUriSchemeIdentifiedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlUnsupportedUriSchemeIdentifiedEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IWebViewControlUnsupportedUriSchemeIdentifiedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IWebViewControlUnsupportedUriSchemeIdentifiedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IWebViewControlUnsupportedUriSchemeIdentifiedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlUnsupportedUriSchemeIdentifiedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWebViewControlUnviewableContentIdentifiedEventArgsImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Referrer(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn MediaType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlUnviewableContentIdentifiedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlUnviewableContentIdentifiedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWebViewControlUnviewableContentIdentifiedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlUnviewableContentIdentifiedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlUnviewableContentIdentifiedEventArgsVtbl {
        unsafe extern "system" fn Uri<Impl: IWebViewControlUnviewableContentIdentifiedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Referrer<Impl: IWebViewControlUnviewableContentIdentifiedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Referrer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: IWebViewControlUnviewableContentIdentifiedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlUnviewableContentIdentifiedEventArgs, BASE_OFFSET>(),
            Uri: Uri::<Impl, IMPL_OFFSET>,
            Referrer: Referrer::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlUnviewableContentIdentifiedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IWebViewControlWebResourceRequestedEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
    fn Request(&mut self) -> ::windows::core::Result<super::Http::HttpRequestMessage>;
    fn SetResponse(&mut self, value: &::core::option::Option<super::Http::HttpResponseMessage>) -> ::windows::core::Result<()>;
    fn Response(&mut self) -> ::windows::core::Result<super::Http::HttpResponseMessage>;
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebViewControlWebResourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControlWebResourceRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Web_Http", feature = "implement_exclusive"))]
impl IWebViewControlWebResourceRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebViewControlWebResourceRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebViewControlWebResourceRequestedEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IWebViewControlWebResourceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Impl: IWebViewControlWebResourceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResponse<Impl: IWebViewControlWebResourceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResponse(&*(&value as *const <super::Http::HttpResponseMessage as ::windows::core::Abi>::Abi as *const <super::Http::HttpResponseMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Response<Impl: IWebViewControlWebResourceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Response() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebViewControlWebResourceRequestedEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            Request: Request::<Impl, IMPL_OFFSET>,
            SetResponse: SetResponse::<Impl, IMPL_OFFSET>,
            Response: Response::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebViewControlWebResourceRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
