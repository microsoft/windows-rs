pub trait IWebApplicationActivation_Impl: Sized {
    fn CancelPendingActivation(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebApplicationActivation {}
impl IWebApplicationActivation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationActivation_Impl, const OFFSET: isize>() -> IWebApplicationActivation_Vtbl {
        unsafe extern "system" fn CancelPendingActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationActivation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelPendingActivation().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CancelPendingActivation: CancelPendingActivation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWebApplicationAuthoringMode_Impl: Sized + super::super::super::Com::IServiceProvider_Impl {
    fn AuthoringClientBinary(&self) -> ::windows::core::Result<super::super::super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWebApplicationAuthoringMode {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWebApplicationAuthoringMode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationAuthoringMode_Impl, const OFFSET: isize>() -> IWebApplicationAuthoringMode_Vtbl {
        unsafe extern "system" fn AuthoringClientBinary<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationAuthoringMode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, designmodedllpath: *mut super::super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthoringClientBinary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(designmodedllpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::Com::IServiceProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            AuthoringClientBinary: AuthoringClientBinary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationAuthoringMode as ::windows::core::Interface>::IID || iid == &<super::super::super::Com::IServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationHost_Impl: Sized {
    fn HWND(&self, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Document(&self) -> ::windows::core::Result<super::super::super::super::Web::MsHtml::IHTMLDocument2>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
    fn Advise(&self, interfaceid: *const ::windows::core::GUID, callback: &::core::option::Option<::windows::core::IUnknown>, cookie: *mut u32) -> ::windows::core::Result<()>;
    fn Unadvise(&self, cookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl ::windows::core::RuntimeName for IWebApplicationHost {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationHost_Impl, const OFFSET: isize>() -> IWebApplicationHost_Vtbl {
        unsafe extern "system" fn HWND<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HWND(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn Document<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmldocument: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Document() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(htmldocument, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, callback: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Advise(::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute(&callback), ::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute_copy(&cookie)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationNavigationEvents_Impl: Sized {
    fn BeforeNavigate(&self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: &::windows::core::PCWSTR, navigationflags: u32, targetframename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn NavigateComplete(&self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn NavigateError(&self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: &::windows::core::PCWSTR, targetframename: &::windows::core::PCWSTR, statuscode: u32) -> ::windows::core::Result<()>;
    fn DocumentComplete(&self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, url: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn DownloadBegin(&self) -> ::windows::core::Result<()>;
    fn DownloadComplete(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl ::windows::core::RuntimeName for IWebApplicationNavigationEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationNavigationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>() -> IWebApplicationNavigationEvents_Vtbl {
        unsafe extern "system" fn BeforeNavigate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: *mut ::core::ffi::c_void, url: ::windows::core::PCWSTR, navigationflags: u32, targetframename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeforeNavigate(::core::mem::transmute(&htmlwindow), ::core::mem::transmute(&url), ::core::mem::transmute_copy(&navigationflags), ::core::mem::transmute(&targetframename)).into()
        }
        unsafe extern "system" fn NavigateComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: *mut ::core::ffi::c_void, url: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NavigateComplete(::core::mem::transmute(&htmlwindow), ::core::mem::transmute(&url)).into()
        }
        unsafe extern "system" fn NavigateError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: *mut ::core::ffi::c_void, url: ::windows::core::PCWSTR, targetframename: ::windows::core::PCWSTR, statuscode: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NavigateError(::core::mem::transmute(&htmlwindow), ::core::mem::transmute(&url), ::core::mem::transmute(&targetframename), ::core::mem::transmute_copy(&statuscode)).into()
        }
        unsafe extern "system" fn DocumentComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: *mut ::core::ffi::c_void, url: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DocumentComplete(::core::mem::transmute(&htmlwindow), ::core::mem::transmute(&url)).into()
        }
        unsafe extern "system" fn DownloadBegin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DownloadBegin().into()
        }
        unsafe extern "system" fn DownloadComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationNavigationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DownloadComplete().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn BeforeScriptExecute(&self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>) -> ::windows::core::Result<()>;
    fn ScriptError(&self, htmlwindow: &::core::option::Option<super::super::super::super::Web::MsHtml::IHTMLWindow2>, scripterror: &::core::option::Option<super::IActiveScriptError>, url: &::windows::core::PCWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl ::windows::core::RuntimeName for IWebApplicationScriptEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationScriptEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationScriptEvents_Impl, const OFFSET: isize>() -> IWebApplicationScriptEvents_Vtbl {
        unsafe extern "system" fn BeforeScriptExecute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationScriptEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeforeScriptExecute(::core::mem::transmute(&htmlwindow)).into()
        }
        unsafe extern "system" fn ScriptError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationScriptEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: *mut ::core::ffi::c_void, scripterror: *mut ::core::ffi::c_void, url: ::windows::core::PCWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScriptError(::core::mem::transmute(&htmlwindow), ::core::mem::transmute(&scripterror), ::core::mem::transmute(&url), ::core::mem::transmute_copy(&errorhandled)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeforeScriptExecute: BeforeScriptExecute::<Identity, Impl, OFFSET>,
            ScriptError: ScriptError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationScriptEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWebApplicationUIEvents_Impl: Sized {
    fn SecurityProblem(&self, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebApplicationUIEvents {}
impl IWebApplicationUIEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationUIEvents_Impl, const OFFSET: isize>() -> IWebApplicationUIEvents_Vtbl {
        unsafe extern "system" fn SecurityProblem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationUIEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SecurityProblem(::core::mem::transmute_copy(&securityproblem), ::core::mem::transmute_copy(&result)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SecurityProblem: SecurityProblem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationUIEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWebApplicationUpdateEvents_Impl: Sized {
    fn OnPaint(&self) -> ::windows::core::Result<()>;
    fn OnCssChanged(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWebApplicationUpdateEvents {}
impl IWebApplicationUpdateEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationUpdateEvents_Impl, const OFFSET: isize>() -> IWebApplicationUpdateEvents_Vtbl {
        unsafe extern "system" fn OnPaint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationUpdateEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPaint().into()
        }
        unsafe extern "system" fn OnCssChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebApplicationUpdateEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCssChanged().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnPaint: OnPaint::<Identity, Impl, OFFSET>,
            OnCssChanged: OnCssChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationUpdateEvents as ::windows::core::Interface>::IID
    }
}
