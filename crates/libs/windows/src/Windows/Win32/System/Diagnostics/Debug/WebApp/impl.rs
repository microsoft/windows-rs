pub trait IWebApplicationActivationImpl: Sized {
    fn CancelPendingActivation();
}
impl ::windows::core::RuntimeName for IWebApplicationActivation {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Debug.WebApp.IWebApplicationActivation";
}
impl IWebApplicationActivationVtbl {
    pub const fn new<Impl: IWebApplicationActivationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebApplicationActivationVtbl {
        unsafe extern "system" fn CancelPendingActivation<Impl: IWebApplicationActivationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelPendingActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebApplicationActivation>, base.5, CancelPendingActivation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebApplicationAuthoringModeImpl: Sized + IServiceProviderImpl {
    fn AuthoringClientBinary();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWebApplicationAuthoringMode {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Debug.WebApp.IWebApplicationAuthoringMode";
}
#[cfg(feature = "Win32_System_Com")]
impl IWebApplicationAuthoringModeVtbl {
    pub const fn new<Impl: IWebApplicationAuthoringModeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebApplicationAuthoringModeVtbl {
        unsafe extern "system" fn AuthoringClientBinary<Impl: IWebApplicationAuthoringModeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, designmodedllpath: *mut super::super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthoringClientBinary(::core::mem::transmute_copy(&designmodedllpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebApplicationAuthoringMode>, base.5, AuthoringClientBinary::<Impl, OFFSET>)
    }
}
pub trait IWebApplicationHostImpl: Sized {
    fn HWND();
    fn Document();
    fn Refresh();
    fn Advise();
    fn Unadvise();
}
impl ::windows::core::RuntimeName for IWebApplicationHost {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Debug.WebApp.IWebApplicationHost";
}
impl IWebApplicationHostVtbl {
    pub const fn new<Impl: IWebApplicationHostImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebApplicationHostVtbl {
        unsafe extern "system" fn HWND<Impl: IWebApplicationHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HWND(&*(&hwnd as *const <super::super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Document<Impl: IWebApplicationHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htmldocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Document(::core::mem::transmute_copy(&htmldocument)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IWebApplicationHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: IWebApplicationHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, callback: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&interfaceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&callback as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), cookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IWebApplicationHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unadvise(cookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebApplicationHost>, base.5, HWND::<Impl, OFFSET>, Document::<Impl, OFFSET>, Refresh::<Impl, OFFSET>, Advise::<Impl, OFFSET>, Unadvise::<Impl, OFFSET>)
    }
}
pub trait IWebApplicationNavigationEventsImpl: Sized {
    fn BeforeNavigate();
    fn NavigateComplete();
    fn NavigateError();
    fn DocumentComplete();
    fn DownloadBegin();
    fn DownloadComplete();
}
impl ::windows::core::RuntimeName for IWebApplicationNavigationEvents {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Debug.WebApp.IWebApplicationNavigationEvents";
}
impl IWebApplicationNavigationEventsVtbl {
    pub const fn new<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebApplicationNavigationEventsVtbl {
        unsafe extern "system" fn BeforeNavigate<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, navigationflags: u32, targetframename: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeforeNavigate(
                &*(&htmlwindow as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::DefaultType>::DefaultType),
                &*(&url as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                navigationflags,
                &*(&targetframename as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigateComplete<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigateComplete(&*(&htmlwindow as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::DefaultType>::DefaultType), &*(&url as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigateError<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, targetframename: super::super::super::super::Foundation::PWSTR, statuscode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigateError(
                &*(&htmlwindow as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::DefaultType>::DefaultType),
                &*(&url as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&targetframename as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                statuscode,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DocumentComplete<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DocumentComplete(&*(&htmlwindow as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::DefaultType>::DefaultType), &*(&url as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadBegin<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DownloadBegin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadComplete<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DownloadComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebApplicationNavigationEvents>, base.5, BeforeNavigate::<Impl, OFFSET>, NavigateComplete::<Impl, OFFSET>, NavigateError::<Impl, OFFSET>, DocumentComplete::<Impl, OFFSET>, DownloadBegin::<Impl, OFFSET>, DownloadComplete::<Impl, OFFSET>)
    }
}
pub trait IWebApplicationScriptEventsImpl: Sized {
    fn BeforeScriptExecute();
    fn ScriptError();
}
impl ::windows::core::RuntimeName for IWebApplicationScriptEvents {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Debug.WebApp.IWebApplicationScriptEvents";
}
impl IWebApplicationScriptEventsVtbl {
    pub const fn new<Impl: IWebApplicationScriptEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebApplicationScriptEventsVtbl {
        unsafe extern "system" fn BeforeScriptExecute<Impl: IWebApplicationScriptEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeforeScriptExecute(&*(&htmlwindow as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScriptError<Impl: IWebApplicationScriptEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, scripterror: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScriptError(
                &*(&htmlwindow as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Web::MsHtml::IHTMLWindow2 as ::windows::core::DefaultType>::DefaultType),
                &*(&scripterror as *const <super::IActiveScriptError as ::windows::core::Abi>::Abi as *const <super::IActiveScriptError as ::windows::core::DefaultType>::DefaultType),
                &*(&url as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&errorhandled as *const <super::super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebApplicationScriptEvents>, base.5, BeforeScriptExecute::<Impl, OFFSET>, ScriptError::<Impl, OFFSET>)
    }
}
pub trait IWebApplicationUIEventsImpl: Sized {
    fn SecurityProblem();
}
impl ::windows::core::RuntimeName for IWebApplicationUIEvents {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Debug.WebApp.IWebApplicationUIEvents";
}
impl IWebApplicationUIEventsVtbl {
    pub const fn new<Impl: IWebApplicationUIEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebApplicationUIEventsVtbl {
        unsafe extern "system" fn SecurityProblem<Impl: IWebApplicationUIEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SecurityProblem(securityproblem, result) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebApplicationUIEvents>, base.5, SecurityProblem::<Impl, OFFSET>)
    }
}
pub trait IWebApplicationUpdateEventsImpl: Sized {
    fn OnPaint();
    fn OnCssChanged();
}
impl ::windows::core::RuntimeName for IWebApplicationUpdateEvents {
    const NAME: &'static str = "Windows.Win32.System.Diagnostics.Debug.WebApp.IWebApplicationUpdateEvents";
}
impl IWebApplicationUpdateEventsVtbl {
    pub const fn new<Impl: IWebApplicationUpdateEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebApplicationUpdateEventsVtbl {
        unsafe extern "system" fn OnPaint<Impl: IWebApplicationUpdateEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnPaint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCssChanged<Impl: IWebApplicationUpdateEventsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnCssChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebApplicationUpdateEvents>, base.5, OnPaint::<Impl, OFFSET>, OnCssChanged::<Impl, OFFSET>)
    }
}
