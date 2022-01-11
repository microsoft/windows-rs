pub trait IWebApplicationActivationImpl: Sized {
    fn CancelPendingActivation();
}
impl IWebApplicationActivationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationActivationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationActivationVtbl {
        unsafe extern "system" fn CancelPendingActivation<Impl: IWebApplicationActivationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CancelPendingActivation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationActivation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWebApplicationAuthoringModeImpl: Sized + IServiceProviderImpl {
    fn AuthoringClientBinary();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWebApplicationAuthoringModeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationAuthoringModeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationAuthoringModeVtbl {
        unsafe extern "system" fn AuthoringClientBinary<Impl: IWebApplicationAuthoringModeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, designmodedllpath: *mut super::super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryService::<Impl, IMPL_OFFSET>, AuthoringClientBinary::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationAuthoringMode as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationHostImpl: Sized {
    fn HWND();
    fn Document();
    fn Refresh();
    fn Advise();
    fn Unadvise();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationHostVtbl {
        unsafe extern "system" fn HWND<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Document<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmldocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Advise<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, callback: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IWebApplicationHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, HWND::<Impl, IMPL_OFFSET>, Document::<Impl, IMPL_OFFSET>, Refresh::<Impl, IMPL_OFFSET>, Advise::<Impl, IMPL_OFFSET>, Unadvise::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationNavigationEventsImpl: Sized {
    fn BeforeNavigate();
    fn NavigateComplete();
    fn NavigateError();
    fn DocumentComplete();
    fn DownloadBegin();
    fn DownloadComplete();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationNavigationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationNavigationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationNavigationEventsVtbl {
        unsafe extern "system" fn BeforeNavigate<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, navigationflags: u32, targetframename: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NavigateComplete<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NavigateError<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, targetframename: super::super::super::super::Foundation::PWSTR, statuscode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DocumentComplete<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadBegin<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DownloadComplete<Impl: IWebApplicationNavigationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, BeforeNavigate::<Impl, IMPL_OFFSET>, NavigateComplete::<Impl, IMPL_OFFSET>, NavigateError::<Impl, IMPL_OFFSET>, DocumentComplete::<Impl, IMPL_OFFSET>, DownloadBegin::<Impl, IMPL_OFFSET>, DownloadComplete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationNavigationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
pub trait IWebApplicationScriptEventsImpl: Sized {
    fn BeforeScriptExecute();
    fn ScriptError();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Web_MsHtml"))]
impl IWebApplicationScriptEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationScriptEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationScriptEventsVtbl {
        unsafe extern "system" fn BeforeScriptExecute<Impl: IWebApplicationScriptEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ScriptError<Impl: IWebApplicationScriptEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htmlwindow: ::windows::core::RawPtr, scripterror: ::windows::core::RawPtr, url: super::super::super::super::Foundation::PWSTR, errorhandled: super::super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, BeforeScriptExecute::<Impl, IMPL_OFFSET>, ScriptError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationScriptEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWebApplicationUIEventsImpl: Sized {
    fn SecurityProblem();
}
impl IWebApplicationUIEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationUIEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationUIEventsVtbl {
        unsafe extern "system" fn SecurityProblem<Impl: IWebApplicationUIEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, securityproblem: u32, result: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SecurityProblem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationUIEvents as ::windows::core::Interface>::IID
    }
}
pub trait IWebApplicationUpdateEventsImpl: Sized {
    fn OnPaint();
    fn OnCssChanged();
}
impl IWebApplicationUpdateEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebApplicationUpdateEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebApplicationUpdateEventsVtbl {
        unsafe extern "system" fn OnPaint<Impl: IWebApplicationUpdateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCssChanged<Impl: IWebApplicationUpdateEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnPaint::<Impl, IMPL_OFFSET>, OnCssChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebApplicationUpdateEvents as ::windows::core::Interface>::IID
    }
}
