pub trait IWebApplicationActivation_Impl: Sized {
    fn CancelPendingActivation(&mut self) -> ::windows::core::Result<()>;
}
impl IWebApplicationActivation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationActivation_Impl, const OFFSET: isize>() -> IWebApplicationActivation_Vtbl {
        unsafe extern "system" fn CancelPendingActivation<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelPendingActivation().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CancelPendingActivation: CancelPendingActivation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWebApplicationAuthoringMode_Impl: Sized + super::super::super::Com::IServiceProvider_Impl {
    fn AuthoringClientBinary(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWebApplicationAuthoringMode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationAuthoringMode_Impl, const OFFSET: isize>() -> IWebApplicationAuthoringMode_Vtbl {
        unsafe extern "system" fn AuthoringClientBinary<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationAuthoringMode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, designmodedllpath: *mut super::super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthoringClientBinary() {
                ::core::result::Result::Ok(ok__) => {
                    *designmodedllpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::super::Com::IServiceProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            AuthoringClientBinary: AuthoringClientBinary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationAuthoringMode as ::windows::core::Interface>::IID || iid == &<super::super::super::Com::IServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationHost_Impl: Sized {
    fn HWND(&mut self, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Document(&mut self) -> ::windows::core::Result<super::super::super::super::Web::MsHtml::IHTMLDocument2>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
    fn Advise(&mut self, interfaceid: *const ::windows::core::GUID, callback: &::core::option::Option<::windows::core::IUnknown>, cookie: *mut u32) -> ::windows::core::Result<()>;
    fn Unadvise(&mut self, cookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationHost_Impl, const OFFSET: isize>() -> IWebApplicationHost_Vtbl {
        unsafe extern "system" fn HWND<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HWND(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn Document<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmldocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Document() {
                ::core::result::Result::Ok(ok__) => {
                    *htmldocument = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Refresh().into()
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, callback: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Advise(::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute(&callback), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&cookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            HWND: HWND::<Identity, Impl, OFFSET>,
            Document: Document::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationNavigationEvents_Impl: Sized {
    fn BeforeNavigate(&mut self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: super::super::super::super::Foundation::PWSTR, navigationflags: u32, targetframename: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn NavigateComplete(&mut self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn NavigateError(&mut self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: super::super::super::super::Foundation::PWSTR, targetframename: super::super::super::super::Foundation::PWSTR, statuscode: u32) -> ::windows::core::Result<()>;
    fn DocumentComplete(&mut self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn DownloadBegin(&mut self) -> ::windows::core::Result<()>;
    fn DownloadComplete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationNavigationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>() -> IWebApplicationNavigationEvents_Vtbl {
        unsafe extern "system" fn BeforeNavigate<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, navigationflags: u32, targetframename: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeforeNavigate(::core::mem::transmute(&htmlwindow), ::core::mem::transmute_copy(&url), ::core::mem::transmute_copy(&navigationflags), ::core::mem::transmute_copy(&targetframename)).into()
        }
        unsafe extern "system" fn NavigateComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NavigateComplete(::core::mem::transmute(&htmlwindow), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn NavigateError<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, targetframename: super::super::super::super::Foundation::PWSTR, statuscode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NavigateError(::core::mem::transmute(&htmlwindow), ::core::mem::transmute_copy(&url), ::core::mem::transmute_copy(&targetframename), ::core::mem::transmute_copy(&statuscode)).into()
        }
        unsafe extern "system" fn DocumentComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DocumentComplete(::core::mem::transmute(&htmlwindow), ::core::mem::transmute_copy(&url)).into()
        }
        unsafe extern "system" fn DownloadBegin<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DownloadBegin().into()
        }
        unsafe extern "system" fn DownloadComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DownloadComplete().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeforeNavigate: BeforeNavigate::<Identity, Impl, OFFSET>,
            NavigateComplete: NavigateComplete::<Identity, Impl, OFFSET>,
            NavigateError: NavigateError::<Identity, Impl, OFFSET>,
            DocumentComplete: DocumentComplete::<Identity, Impl, OFFSET>,
            DownloadBegin: DownloadBegin::<Identity, Impl, OFFSET>,
            DownloadComplete: DownloadComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationNavigationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationScriptEvents_Impl: Sized {
    fn BeforeScriptExecute(&mut self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>) -> ::windows::core::Result<()>;
    fn ScriptError(&mut self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, scripterror: &::core::option::Option<super::IActiveScriptError>, url: super::super::super::super::Foundation::PWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationScriptEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationScriptEvents_Impl, const OFFSET: isize>() -> IWebApplicationScriptEvents_Vtbl {
        unsafe extern "system" fn BeforeScriptExecute<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationScriptEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeforeScriptExecute(::core::mem::transmute(&htmlwindow)).into()
        }
        unsafe extern "system" fn ScriptError<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationScriptEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, scripterror: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScriptError(::core::mem::transmute(&htmlwindow), ::core::mem::transmute(&scripterror), ::core::mem::transmute_copy(&url), ::core::mem::transmute_copy(&errorhandled)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeforeScriptExecute: BeforeScriptExecute::<Identity, Impl, OFFSET>,
            ScriptError: ScriptError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationScriptEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWebApplicationUIEvents_Impl: Sized {
    fn SecurityProblem(&mut self, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IWebApplicationUIEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationUIEvents_Impl, const OFFSET: isize>() -> IWebApplicationUIEvents_Vtbl {
        unsafe extern "system" fn SecurityProblem<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationUIEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SecurityProblem(::core::mem::transmute_copy(&securityproblem), ::core::mem::transmute_copy(&result)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SecurityProblem: SecurityProblem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationUIEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWebApplicationUpdateEvents_Impl: Sized {
    fn OnPaint(&mut self) -> ::windows::core::Result<()>;
    fn OnCssChanged(&mut self) -> ::windows::core::Result<()>;
}
impl IWebApplicationUpdateEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationUpdateEvents_Impl, const OFFSET: isize>() -> IWebApplicationUpdateEvents_Vtbl {
        unsafe extern "system" fn OnPaint<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationUpdateEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPaint().into()
        }
        unsafe extern "system" fn OnCssChanged<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationUpdateEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCssChanged().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnPaint: OnPaint::<Identity, Impl, OFFSET>,
            OnCssChanged: OnCssChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationUpdateEvents as ::windows::core::Interface>::IID
    }
}
