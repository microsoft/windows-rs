pub trait IWebApplicationActivationImpl: Sized {
    fn CancelPendingActivation(&mut self) -> ::windows::core::Result<()>;
}
impl IWebApplicationActivationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationActivationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationActivationVtbl {
        unsafe extern "system" fn CancelPendingActivation<Impl: IWebApplicationActivationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelPendingActivation().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CancelPendingActivation: CancelPendingActivation::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWebApplicationAuthoringModeImpl: Sized + IServiceProviderImpl {
    fn AuthoringClientBinary(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWebApplicationAuthoringModeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationAuthoringModeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationAuthoringModeVtbl {
        unsafe extern "system" fn AuthoringClientBinary<Impl: IWebApplicationAuthoringModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, designmodedllpath: *mut super::super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthoringClientBinary() {
                ::core::result::Result::Ok(ok__) => {
                    *designmodedllpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IServiceProviderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), AuthoringClientBinary: AuthoringClientBinary::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationAuthoringMode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationHostImpl: Sized {
    fn HWND(&mut self, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Document(&mut self) -> ::windows::core::Result<super::super::super::super::Web::MsHtml::IHTMLDocument2>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Advise(&mut self, interfaceid: *const ::windows::core::GUID, callback: ::core::option::Option<::windows::core::IUnknown>, cookie: *mut u32) -> ::windows::core::Result<()>;
    fn Unadvise(&mut self, cookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationHostVtbl {
        unsafe extern "system" fn HWND<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HWND(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn Document<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmldocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Document() {
                ::core::result::Result::Ok(ok__) => {
                    *htmldocument = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Advise<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, callback: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Advise(::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute(&callback), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn Unadvise<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&cookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            HWND: HWND::<Impl, IMPL_OFFSET>,
            Document: Document::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
            Advise: Advise::<Impl, IMPL_OFFSET>,
            Unadvise: Unadvise::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationNavigationEventsImpl: Sized {
    fn BeforeNavigate(&mut self, htmlwindow: ::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: super::super::super::super::Foundation::PWSTR, navigationflags: u32, targetframename: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn NavigateComplete(&mut self, htmlwindow: ::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn NavigateError(&mut self, htmlwindow: ::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: super::super::super::super::Foundation::PWSTR, targetframename: super::super::super::super::Foundation::PWSTR, statuscode: u32) -> ::windows::core::Result<()>;
    fn DocumentComplete(&mut self, htmlwindow: ::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DownloadBegin(&mut self) -> ::windows::core::Result<()>;
    fn DownloadComplete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationNavigationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationNavigationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationNavigationEventsVtbl {
        unsafe extern "system" fn BeforeNavigate<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, navigationflags: u32, targetframename: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeforeNavigate(::core::mem::transmute(&htmlwindow), ::core::mem::transmute_copy(&url), ::core::mem::transmute_copy(&navigationflags), ::core::mem::transmute_copy(&targetframename)).into()
        }
        unsafe extern "system" fn NavigateComplete<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NavigateComplete(::core::mem::transmute(&htmlwindow), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn NavigateError<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, targetframename: super::super::super::super::Foundation::PWSTR, statuscode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NavigateError(::core::mem::transmute(&htmlwindow), ::core::mem::transmute_copy(&url), ::core::mem::transmute_copy(&targetframename), ::core::mem::transmute_copy(&statuscode)).into()
        }
        unsafe extern "system" fn DocumentComplete<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DocumentComplete(::core::mem::transmute(&htmlwindow), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn DownloadBegin<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DownloadBegin().into()
        }
        unsafe extern "system" fn DownloadComplete<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DownloadComplete().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeforeNavigate: BeforeNavigate::<Impl, IMPL_OFFSET>,
            NavigateComplete: NavigateComplete::<Impl, IMPL_OFFSET>,
            NavigateError: NavigateError::<Impl, IMPL_OFFSET>,
            DocumentComplete: DocumentComplete::<Impl, IMPL_OFFSET>,
            DownloadBegin: DownloadBegin::<Impl, IMPL_OFFSET>,
            DownloadComplete: DownloadComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationNavigationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationScriptEventsImpl: Sized {
    fn BeforeScriptExecute(&mut self, htmlwindow: ::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>) -> ::windows::core::Result<()>;
    fn ScriptError(&mut self, htmlwindow: ::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, scripterror: ::core::option::Option<super::IActiveScriptError>, url: super::super::super::super::Foundation::PWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationScriptEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationScriptEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationScriptEventsVtbl {
        unsafe extern "system" fn BeforeScriptExecute<Impl: IWebApplicationScriptEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeforeScriptExecute(::core::mem::transmute(&htmlwindow)).into()
        }
        unsafe extern "system" fn ScriptError<Impl: IWebApplicationScriptEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, scripterror: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScriptError(::core::mem::transmute(&htmlwindow), ::core::mem::transmute(&scripterror), ::core::mem::transmute_copy(&url), ::core::mem::transmute_copy(&errorhandled)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeforeScriptExecute: BeforeScriptExecute::<Impl, IMPL_OFFSET>,
            ScriptError: ScriptError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationScriptEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWebApplicationUIEventsImpl: Sized {
    fn SecurityProblem(&mut self, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IWebApplicationUIEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationUIEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationUIEventsVtbl {
        unsafe extern "system" fn SecurityProblem<Impl: IWebApplicationUIEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SecurityProblem(::core::mem::transmute_copy(&securityproblem), ::core::mem::transmute_copy(&result)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SecurityProblem: SecurityProblem::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationUIEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWebApplicationUpdateEventsImpl: Sized {
    fn OnPaint(&mut self) -> ::windows::core::Result<()>;
    fn OnCssChanged(&mut self) -> ::windows::core::Result<()>;
}
impl IWebApplicationUpdateEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationUpdateEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationUpdateEventsVtbl {
        unsafe extern "system" fn OnPaint<Impl: IWebApplicationUpdateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPaint().into()
        }
        unsafe extern "system" fn OnCssChanged<Impl: IWebApplicationUpdateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCssChanged().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnPaint: OnPaint::<Impl, IMPL_OFFSET>,
            OnCssChanged: OnCssChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationUpdateEvents as ::windows::core::Interface>::IID
    }
}
