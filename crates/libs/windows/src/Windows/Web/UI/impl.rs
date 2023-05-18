#[doc = "*Required features: `\"Web_UI\"`, `\"ApplicationModel_DataTransfer\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`, `\"UI\"`, `\"Web_Http\"`, `\"implement\"`*"]
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
pub trait IWebViewControl_Impl: Sized {
    fn Source(&self) -> ::windows_core::Result<super::super::Foundation::Uri>;
    fn SetSource(&self, source: ::core::option::Option<&super::super::Foundation::Uri>) -> ::windows_core::Result<()>;
    fn DocumentTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn CanGoBack(&self) -> ::windows_core::Result<bool>;
    fn CanGoForward(&self) -> ::windows_core::Result<bool>;
    fn SetDefaultBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows_core::Result<()>;
    fn DefaultBackgroundColor(&self) -> ::windows_core::Result<super::super::UI::Color>;
    fn ContainsFullScreenElement(&self) -> ::windows_core::Result<bool>;
    fn Settings(&self) -> ::windows_core::Result<WebViewControlSettings>;
    fn DeferredPermissionRequests(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>>;
    fn GoForward(&self) -> ::windows_core::Result<()>;
    fn GoBack(&self) -> ::windows_core::Result<()>;
    fn Refresh(&self) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
    fn Navigate(&self, source: ::core::option::Option<&super::super::Foundation::Uri>) -> ::windows_core::Result<()>;
    fn NavigateToString(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn NavigateToLocalStreamUri(&self, source: ::core::option::Option<&super::super::Foundation::Uri>, streamresolver: ::core::option::Option<&super::IUriToStreamResolver>) -> ::windows_core::Result<()>;
    fn NavigateWithHttpRequestMessage(&self, requestmessage: ::core::option::Option<&super::Http::HttpRequestMessage>) -> ::windows_core::Result<()>;
    fn InvokeScriptAsync(&self, scriptname: &::windows_core::HSTRING, arguments: ::core::option::Option<&super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>;
    fn CapturePreviewToStreamAsync(&self, stream: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>>;
    fn BuildLocalStreamUri(&self, contentidentifier: &::windows_core::HSTRING, relativepath: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Uri>;
    fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<WebViewControlDeferredPermissionRequest>) -> ::windows_core::Result<()>;
    fn NavigationStarting(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn ContentLoading(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn DOMContentLoaded(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDOMContentLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn NavigationCompleted(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FrameNavigationStarting(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FrameContentLoading(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameContentLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FrameDOMContentLoaded(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameDOMContentLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FrameNavigationCompleted(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn ScriptNotify(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScriptNotify(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn LongRunningScriptDetected(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLongRunningScriptDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn UnsafeContentWarningDisplaying(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsafeContentWarningDisplaying(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn UnviewableContentIdentified(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnviewableContentIdentified(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PermissionRequested(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePermissionRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn UnsupportedUriSchemeIdentified(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsupportedUriSchemeIdentified(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn NewWindowRequested(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWindowRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn ContainsFullScreenElementChanged(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContainsFullScreenElementChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn WebResourceRequested(&self, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveWebResourceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl ::windows_core::RuntimeName for IWebViewControl {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl IWebViewControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>() -> IWebViewControl_Vtbl {
        unsafe extern "system" fn Source<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Source() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSource(::windows_core::from_raw_borrowed(&source)).into()
        }
        unsafe extern "system" fn DocumentTitle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DocumentTitle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanGoBack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanGoBack() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanGoForward<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanGoForward() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultBackgroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultBackgroundColor(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DefaultBackgroundColor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainsFullScreenElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContainsFullScreenElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Settings() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeferredPermissionRequests<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeferredPermissionRequests() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GoForward<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GoForward().into()
        }
        unsafe extern "system" fn GoBack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GoBack().into()
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Navigate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Navigate(::windows_core::from_raw_borrowed(&source)).into()
        }
        unsafe extern "system" fn NavigateToString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NavigateToString(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn NavigateToLocalStreamUri<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, streamresolver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NavigateToLocalStreamUri(::windows_core::from_raw_borrowed(&source), ::windows_core::from_raw_borrowed(&streamresolver)).into()
        }
        unsafe extern "system" fn NavigateWithHttpRequestMessage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestmessage: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NavigateWithHttpRequestMessage(::windows_core::from_raw_borrowed(&requestmessage)).into()
        }
        unsafe extern "system" fn InvokeScriptAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scriptname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InvokeScriptAsync(::core::mem::transmute(&scriptname), ::windows_core::from_raw_borrowed(&arguments)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePreviewToStreamAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CapturePreviewToStreamAsync(::windows_core::from_raw_borrowed(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaptureSelectedContentToDataPackageAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CaptureSelectedContentToDataPackageAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildLocalStreamUri<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentidentifier: ::std::mem::MaybeUninit<::windows_core::HSTRING>, relativepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BuildLocalStreamUri(::core::mem::transmute(&contentidentifier), ::core::mem::transmute(&relativepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferredPermissionRequestById<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: u32, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeferredPermissionRequestById(id, ::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn NavigationStarting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NavigationStarting(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationStarting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveNavigationStarting(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ContentLoading<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContentLoading(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContentLoading<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveContentLoading(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn DOMContentLoaded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DOMContentLoaded(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDOMContentLoaded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveDOMContentLoaded(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn NavigationCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NavigationCompleted(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNavigationCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveNavigationCompleted(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameNavigationStarting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FrameNavigationStarting(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationStarting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFrameNavigationStarting(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameContentLoading<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FrameContentLoading(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameContentLoading<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFrameContentLoading(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameDOMContentLoaded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FrameDOMContentLoaded(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameDOMContentLoaded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFrameDOMContentLoaded(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn FrameNavigationCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FrameNavigationCompleted(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFrameNavigationCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveFrameNavigationCompleted(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ScriptNotify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScriptNotify(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScriptNotify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveScriptNotify(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn LongRunningScriptDetected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LongRunningScriptDetected(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLongRunningScriptDetected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLongRunningScriptDetected(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UnsafeContentWarningDisplaying<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnsafeContentWarningDisplaying(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnsafeContentWarningDisplaying<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveUnsafeContentWarningDisplaying(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UnviewableContentIdentified<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnviewableContentIdentified(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnviewableContentIdentified<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveUnviewableContentIdentified(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn PermissionRequested<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PermissionRequested(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePermissionRequested<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePermissionRequested(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn UnsupportedUriSchemeIdentified<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UnsupportedUriSchemeIdentified(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnsupportedUriSchemeIdentified<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveUnsupportedUriSchemeIdentified(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn NewWindowRequested<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NewWindowRequested(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNewWindowRequested<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveNewWindowRequested(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn ContainsFullScreenElementChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContainsFullScreenElementChanged(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContainsFullScreenElementChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveContainsFullScreenElementChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn WebResourceRequested<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WebResourceRequested(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWebResourceRequested<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveWebResourceRequested(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IWebViewControl, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWebViewControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Web_UI\"`, `\"implement\"`*"]
pub trait IWebViewControl2_Impl: Sized {
    fn AddInitializeScript(&self, script: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWebViewControl2 {
    const NAME: &'static str = "Windows.Web.UI.IWebViewControl2";
}
impl IWebViewControl2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl2_Impl, const OFFSET: isize>() -> IWebViewControl2_Vtbl {
        unsafe extern "system" fn AddInitializeScript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebViewControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, script: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddInitializeScript(::core::mem::transmute(&script)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IWebViewControl2, OFFSET>(),
            AddInitializeScript: AddInitializeScript::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWebViewControl2 as ::windows_core::ComInterface>::IID
    }
}
