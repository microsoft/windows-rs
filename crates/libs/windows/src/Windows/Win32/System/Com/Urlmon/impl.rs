pub trait IBindCallbackRedirect_Impl: Sized {
    fn Redirect(&self, lpcurl: &::windows::core::PCWSTR) -> ::windows::core::Result<i16>;
}
impl ::windows::core::RuntimeName for IBindCallbackRedirect {}
impl IBindCallbackRedirect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCallbackRedirect_Impl, const OFFSET: isize>() -> IBindCallbackRedirect_Vtbl {
        unsafe extern "system" fn Redirect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCallbackRedirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcurl: ::windows::core::PCWSTR, vbcancel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Redirect(::core::mem::transmute(&lpcurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vbcancel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Redirect: Redirect::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindCallbackRedirect as ::windows::core::Interface>::IID
    }
}
pub trait IBindHttpSecurity_Impl: Sized {
    fn GetIgnoreCertMask(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IBindHttpSecurity {}
impl IBindHttpSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindHttpSecurity_Impl, const OFFSET: isize>() -> IBindHttpSecurity_Vtbl {
        unsafe extern "system" fn GetIgnoreCertMask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindHttpSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwignorecertmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIgnoreCertMask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwignorecertmask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetIgnoreCertMask: GetIgnoreCertMask::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindHttpSecurity as ::windows::core::Interface>::IID
    }
}
pub trait IBindProtocol_Impl: Sized {
    fn CreateBinding(&self, szurl: &::windows::core::PCWSTR, pbc: &::core::option::Option<super::IBindCtx>) -> ::windows::core::Result<super::IBinding>;
}
impl ::windows::core::RuntimeName for IBindProtocol {}
impl IBindProtocol_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindProtocol_Impl, const OFFSET: isize>() -> IBindProtocol_Vtbl {
        unsafe extern "system" fn CreateBinding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: ::windows::core::PCWSTR, pbc: *mut ::core::ffi::c_void, ppb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateBinding(::core::mem::transmute(&szurl), ::core::mem::transmute(&pbc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateBinding: CreateBinding::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindProtocol as ::windows::core::Interface>::IID
    }
}
pub trait ICatalogFileInfo_Impl: Sized {
    fn GetCatalogFile(&self) -> ::windows::core::Result<::windows::core::PSTR>;
    fn GetJavaTrust(&self, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICatalogFileInfo {}
impl ICatalogFileInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogFileInfo_Impl, const OFFSET: isize>() -> ICatalogFileInfo_Vtbl {
        unsafe extern "system" fn GetCatalogFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogFileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcatalogfile: *mut ::windows::core::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCatalogFile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcatalogfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJavaTrust<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogFileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetJavaTrust(::core::mem::transmute_copy(&ppjavatrust)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCatalogFile: GetCatalogFile::<Identity, Impl, OFFSET>,
            GetJavaTrust: GetJavaTrust::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalogFileInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICodeInstall_Impl: Sized + IWindowForBindingUI_Impl {
    fn OnCodeInstallProblem(&self, ulstatuscode: u32, szdestination: &::windows::core::PCWSTR, szsource: &::windows::core::PCWSTR, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICodeInstall {}
#[cfg(feature = "Win32_Foundation")]
impl ICodeInstall_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICodeInstall_Impl, const OFFSET: isize>() -> ICodeInstall_Vtbl {
        unsafe extern "system" fn OnCodeInstallProblem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICodeInstall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szdestination: ::windows::core::PCWSTR, szsource: ::windows::core::PCWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCodeInstallProblem(::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute(&szdestination), ::core::mem::transmute(&szsource), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: IWindowForBindingUI_Vtbl::new::<Identity, Impl, OFFSET>(), OnCodeInstallProblem: OnCodeInstallProblem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICodeInstall as ::windows::core::Interface>::IID || iid == &<IWindowForBindingUI as ::windows::core::Interface>::IID
    }
}
pub trait IDataFilter_Impl: Sized {
    fn DoEncode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn DoDecode(&self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SetEncodingLevel(&self, dwenclevel: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDataFilter {}
impl IDataFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataFilter_Impl, const OFFSET: isize>() -> IDataFilter_Vtbl {
        unsafe extern "system" fn DoEncode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoEncode(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&linbuffersize), ::core::mem::transmute_copy(&pbinbuffer), ::core::mem::transmute_copy(&loutbuffersize), ::core::mem::transmute_copy(&pboutbuffer), ::core::mem::transmute_copy(&linbytesavailable), ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn DoDecode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoDecode(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&linbuffersize), ::core::mem::transmute_copy(&pbinbuffer), ::core::mem::transmute_copy(&loutbuffersize), ::core::mem::transmute_copy(&pboutbuffer), ::core::mem::transmute_copy(&linbytesavailable), ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetEncodingLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenclevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEncodingLevel(::core::mem::transmute_copy(&dwenclevel)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DoEncode: DoEncode::<Identity, Impl, OFFSET>,
            DoDecode: DoDecode::<Identity, Impl, OFFSET>,
            SetEncodingLevel: SetEncodingLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataFilter as ::windows::core::Interface>::IID
    }
}
pub trait IEncodingFilterFactory_Impl: Sized {
    fn FindBestFilter(&self, pwzcodein: &::windows::core::PCWSTR, pwzcodeout: &::windows::core::PCWSTR, info: &DATAINFO) -> ::windows::core::Result<IDataFilter>;
    fn GetDefaultFilter(&self, pwzcodein: &::windows::core::PCWSTR, pwzcodeout: &::windows::core::PCWSTR) -> ::windows::core::Result<IDataFilter>;
}
impl ::windows::core::RuntimeName for IEncodingFilterFactory {}
impl IEncodingFilterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEncodingFilterFactory_Impl, const OFFSET: isize>() -> IEncodingFilterFactory_Vtbl {
        unsafe extern "system" fn FindBestFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEncodingFilterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzcodein: ::windows::core::PCWSTR, pwzcodeout: ::windows::core::PCWSTR, info: DATAINFO, ppdf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindBestFilter(::core::mem::transmute(&pwzcodein), ::core::mem::transmute(&pwzcodeout), ::core::mem::transmute(&info)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdf, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEncodingFilterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzcodein: ::windows::core::PCWSTR, pwzcodeout: ::windows::core::PCWSTR, ppdf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultFilter(::core::mem::transmute(&pwzcodein), ::core::mem::transmute(&pwzcodeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdf, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FindBestFilter: FindBestFilter::<Identity, Impl, OFFSET>,
            GetDefaultFilter: GetDefaultFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEncodingFilterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetBindHandle_Impl: Sized {
    fn GetBindHandle(&self, enumrequestedhandle: BINDHANDLETYPES) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IGetBindHandle {}
#[cfg(feature = "Win32_Foundation")]
impl IGetBindHandle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetBindHandle_Impl, const OFFSET: isize>() -> IGetBindHandle_Vtbl {
        unsafe extern "system" fn GetBindHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetBindHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumrequestedhandle: BINDHANDLETYPES, prethandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBindHandle(::core::mem::transmute_copy(&enumrequestedhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prethandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetBindHandle: GetBindHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetBindHandle as ::windows::core::Interface>::IID
    }
}
pub trait IHttpNegotiate_Impl: Sized {
    fn BeginningTransaction(&self, szurl: &::windows::core::PCWSTR, szheaders: &::windows::core::PCWSTR, dwreserved: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn OnResponse(&self, dwresponsecode: u32, szresponseheaders: &::windows::core::PCWSTR, szrequestheaders: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IHttpNegotiate {}
impl IHttpNegotiate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHttpNegotiate_Impl, const OFFSET: isize>() -> IHttpNegotiate_Vtbl {
        unsafe extern "system" fn BeginningTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHttpNegotiate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: ::windows::core::PCWSTR, szheaders: ::windows::core::PCWSTR, dwreserved: u32, pszadditionalheaders: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginningTransaction(::core::mem::transmute(&szurl), ::core::mem::transmute(&szheaders), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszadditionalheaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResponse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHttpNegotiate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwresponsecode: u32, szresponseheaders: ::windows::core::PCWSTR, szrequestheaders: ::windows::core::PCWSTR, pszadditionalrequestheaders: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OnResponse(::core::mem::transmute_copy(&dwresponsecode), ::core::mem::transmute(&szresponseheaders), ::core::mem::transmute(&szrequestheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszadditionalrequestheaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeginningTransaction: BeginningTransaction::<Identity, Impl, OFFSET>,
            OnResponse: OnResponse::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNegotiate as ::windows::core::Interface>::IID
    }
}
pub trait IHttpNegotiate2_Impl: Sized + IHttpNegotiate_Impl {
    fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHttpNegotiate2 {}
impl IHttpNegotiate2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHttpNegotiate2_Impl, const OFFSET: isize>() -> IHttpNegotiate2_Vtbl {
        unsafe extern "system" fn GetRootSecurityId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHttpNegotiate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRootSecurityId(::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: IHttpNegotiate_Vtbl::new::<Identity, Impl, OFFSET>(), GetRootSecurityId: GetRootSecurityId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNegotiate2 as ::windows::core::Interface>::IID || iid == &<IHttpNegotiate as ::windows::core::Interface>::IID
    }
}
pub trait IHttpNegotiate3_Impl: Sized + IHttpNegotiate_Impl + IHttpNegotiate2_Impl {
    fn GetSerializedClientCertContext(&self, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IHttpNegotiate3 {}
impl IHttpNegotiate3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHttpNegotiate3_Impl, const OFFSET: isize>() -> IHttpNegotiate3_Vtbl {
        unsafe extern "system" fn GetSerializedClientCertContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHttpNegotiate3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSerializedClientCertContext(::core::mem::transmute_copy(&ppbcert), ::core::mem::transmute_copy(&pcbcert)).into()
        }
        Self {
            base__: IHttpNegotiate2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSerializedClientCertContext: GetSerializedClientCertContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNegotiate3 as ::windows::core::Interface>::IID || iid == &<IHttpNegotiate as ::windows::core::Interface>::IID || iid == &<IHttpNegotiate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHttpSecurity_Impl: Sized + IWindowForBindingUI_Impl {
    fn OnSecurityProblem(&self, dwproblem: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IHttpSecurity {}
#[cfg(feature = "Win32_Foundation")]
impl IHttpSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHttpSecurity_Impl, const OFFSET: isize>() -> IHttpSecurity_Vtbl {
        unsafe extern "system" fn OnSecurityProblem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHttpSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproblem: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSecurityProblem(::core::mem::transmute_copy(&dwproblem)).into()
        }
        Self { base__: IWindowForBindingUI_Vtbl::new::<Identity, Impl, OFFSET>(), OnSecurityProblem: OnSecurityProblem::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpSecurity as ::windows::core::Interface>::IID || iid == &<IWindowForBindingUI as ::windows::core::Interface>::IID
    }
}
pub trait IInternet_Impl: Sized {}
impl ::windows::core::RuntimeName for IInternet {}
impl IInternet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternet_Impl, const OFFSET: isize>() -> IInternet_Vtbl {
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInternetBindInfo_Impl: Sized {
    fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::Result<()>;
    fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut ::windows::core::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IInternetBindInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IInternetBindInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetBindInfo_Impl, const OFFSET: isize>() -> IInternetBindInfo_Vtbl {
        unsafe extern "system" fn GetBindInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetBindInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindInfo(::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo)).into()
        }
        unsafe extern "system" fn GetBindString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetBindInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstringtype: u32, ppwzstr: *mut ::windows::core::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindString(::core::mem::transmute_copy(&ulstringtype), ::core::mem::transmute_copy(&ppwzstr), ::core::mem::transmute_copy(&cel), ::core::mem::transmute_copy(&pcelfetched)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBindInfo: GetBindInfo::<Identity, Impl, OFFSET>,
            GetBindString: GetBindString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetBindInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInternetBindInfoEx_Impl: Sized + IInternetBindInfo_Impl {
    fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IInternetBindInfoEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IInternetBindInfoEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetBindInfoEx_Impl, const OFFSET: isize>() -> IInternetBindInfoEx_Vtbl {
        unsafe extern "system" fn GetBindInfoEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetBindInfoEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindInfoEx(::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&grfbindf2), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: IInternetBindInfo_Vtbl::new::<Identity, Impl, OFFSET>(), GetBindInfoEx: GetBindInfoEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetBindInfoEx as ::windows::core::Interface>::IID || iid == &<IInternetBindInfo as ::windows::core::Interface>::IID
    }
}
pub trait IInternetHostSecurityManager_Impl: Sized {
    fn GetSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
    fn ProcessUrlAction(&self, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn QueryCustomPolicy(&self, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternetHostSecurityManager {}
impl IInternetHostSecurityManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetHostSecurityManager_Impl, const OFFSET: isize>() -> IInternetHostSecurityManager_Vtbl {
        unsafe extern "system" fn GetSecurityId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSecurityId(::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn ProcessUrlAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessUrlAction(::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryCustomPolicy(::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSecurityId: GetSecurityId::<Identity, Impl, OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Identity, Impl, OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetHostSecurityManager as ::windows::core::Interface>::IID
    }
}
pub trait IInternetPriority_Impl: Sized {
    fn SetPriority(&self, npriority: i32) -> ::windows::core::Result<()>;
    fn GetPriority(&self) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IInternetPriority {}
impl IInternetPriority_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetPriority_Impl, const OFFSET: isize>() -> IInternetPriority_Vtbl {
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetPriority_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&npriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetPriority_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetPriority as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocol_Impl: Sized + IInternetProtocolRoot_Impl {
    fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()>;
    fn Seek(&self, dlibmove: i64, dworigin: u32) -> ::windows::core::Result<u64>;
    fn LockRequest(&self, dwoptions: u32) -> ::windows::core::Result<()>;
    fn UnlockRequest(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInternetProtocol {}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocol_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: isize>() -> IInternetProtocol_Vtbl {
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Seek(::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&dworigin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plibnewposition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockRequest(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn UnlockRequest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlockRequest().into()
        }
        Self {
            base__: IInternetProtocolRoot_Vtbl::new::<Identity, Impl, OFFSET>(),
            Read: Read::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            LockRequest: LockRequest::<Identity, Impl, OFFSET>,
            UnlockRequest: UnlockRequest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocol as ::windows::core::Interface>::IID || iid == &<IInternetProtocolRoot as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolEx_Impl: Sized + IInternetProtocolRoot_Impl + IInternetProtocol_Impl {
    fn StartEx(&self, puri: &::core::option::Option<super::IUri>, poiprotsink: &::core::option::Option<IInternetProtocolSink>, poibindinfo: &::core::option::Option<IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInternetProtocolEx {}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolEx_Impl, const OFFSET: isize>() -> IInternetProtocolEx_Vtbl {
        unsafe extern "system" fn StartEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, poiprotsink: *mut ::core::ffi::c_void, poibindinfo: *mut ::core::ffi::c_void, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartEx(::core::mem::transmute(&puri), ::core::mem::transmute(&poiprotsink), ::core::mem::transmute(&poibindinfo), ::core::mem::transmute_copy(&grfpi), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: IInternetProtocol_Vtbl::new::<Identity, Impl, OFFSET>(), StartEx: StartEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolEx as ::windows::core::Interface>::IID || iid == &<IInternetProtocolRoot as ::windows::core::Interface>::IID || iid == &<IInternetProtocol as ::windows::core::Interface>::IID
    }
}
pub trait IInternetProtocolInfo_Impl: Sized {
    fn ParseUrl(&self, pwzurl: &::windows::core::PCWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: ::windows::core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn CombineUrl(&self, pwzbaseurl: &::windows::core::PCWSTR, pwzrelativeurl: &::windows::core::PCWSTR, dwcombineflags: u32, pwzresult: &::windows::core::PCWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn CompareUrl(&self, pwzurl1: &::windows::core::PCWSTR, pwzurl2: &::windows::core::PCWSTR, dwcompareflags: u32) -> ::windows::core::Result<()>;
    fn QueryInfo(&self, pwzurl: &::windows::core::PCWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternetProtocolInfo {}
impl IInternetProtocolInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: isize>() -> IInternetProtocolInfo_Vtbl {
        unsafe extern "system" fn ParseUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl: ::windows::core::PCWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: ::windows::core::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseUrl(::core::mem::transmute(&pwzurl), ::core::mem::transmute_copy(&parseaction), ::core::mem::transmute_copy(&dwparseflags), ::core::mem::transmute_copy(&pwzresult), ::core::mem::transmute_copy(&cchresult), ::core::mem::transmute_copy(&pcchresult), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn CombineUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzbaseurl: ::windows::core::PCWSTR, pwzrelativeurl: ::windows::core::PCWSTR, dwcombineflags: u32, pwzresult: ::windows::core::PCWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CombineUrl(::core::mem::transmute(&pwzbaseurl), ::core::mem::transmute(&pwzrelativeurl), ::core::mem::transmute_copy(&dwcombineflags), ::core::mem::transmute(&pwzresult), ::core::mem::transmute_copy(&cchresult), ::core::mem::transmute_copy(&pcchresult), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn CompareUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl1: ::windows::core::PCWSTR, pwzurl2: ::windows::core::PCWSTR, dwcompareflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompareUrl(::core::mem::transmute(&pwzurl1), ::core::mem::transmute(&pwzurl2), ::core::mem::transmute_copy(&dwcompareflags)).into()
        }
        unsafe extern "system" fn QueryInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl: ::windows::core::PCWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryInfo(::core::mem::transmute(&pwzurl), ::core::mem::transmute_copy(&oueryoption), ::core::mem::transmute_copy(&dwqueryflags), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcbbuf), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ParseUrl: ParseUrl::<Identity, Impl, OFFSET>,
            CombineUrl: CombineUrl::<Identity, Impl, OFFSET>,
            CompareUrl: CompareUrl::<Identity, Impl, OFFSET>,
            QueryInfo: QueryInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolRoot_Impl: Sized {
    fn Start(&self, szurl: &::windows::core::PCWSTR, poiprotsink: &::core::option::Option<IInternetProtocolSink>, poibindinfo: &::core::option::Option<IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()>;
    fn Abort(&self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()>;
    fn Terminate(&self, dwoptions: u32) -> ::windows::core::Result<()>;
    fn Suspend(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInternetProtocolRoot {}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolRoot_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>() -> IInternetProtocolRoot_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: ::windows::core::PCWSTR, poiprotsink: *mut ::core::ffi::c_void, poibindinfo: *mut ::core::ffi::c_void, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute(&szurl), ::core::mem::transmute(&poiprotsink), ::core::mem::transmute(&poibindinfo), ::core::mem::transmute_copy(&grfpi), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Continue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Continue(::core::mem::transmute_copy(&pprotocoldata)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort(::core::mem::transmute_copy(&hrreason), ::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn Suspend<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Suspend().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Continue: Continue::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolRoot as ::windows::core::Interface>::IID
    }
}
pub trait IInternetProtocolSink_Impl: Sized {
    fn Switch(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()>;
    fn ReportProgress(&self, ulstatuscode: u32, szstatustext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn ReportData(&self, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()>;
    fn ReportResult(&self, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternetProtocolSink {}
impl IInternetProtocolSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: isize>() -> IInternetProtocolSink_Vtbl {
        unsafe extern "system" fn Switch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Switch(::core::mem::transmute_copy(&pprotocoldata)).into()
        }
        unsafe extern "system" fn ReportProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportProgress(::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute(&szstatustext)).into()
        }
        unsafe extern "system" fn ReportData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportData(::core::mem::transmute_copy(&grfbscf), ::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax)).into()
        }
        unsafe extern "system" fn ReportResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportResult(::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&dwerror), ::core::mem::transmute(&szresult)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Switch: Switch::<Identity, Impl, OFFSET>,
            ReportProgress: ReportProgress::<Identity, Impl, OFFSET>,
            ReportData: ReportData::<Identity, Impl, OFFSET>,
            ReportResult: ReportResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolSink as ::windows::core::Interface>::IID
    }
}
pub trait IInternetProtocolSinkStackable_Impl: Sized {
    fn SwitchSink(&self, poiprotsink: &::core::option::Option<IInternetProtocolSink>) -> ::windows::core::Result<()>;
    fn CommitSwitch(&self) -> ::windows::core::Result<()>;
    fn RollbackSwitch(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternetProtocolSinkStackable {}
impl IInternetProtocolSinkStackable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>() -> IInternetProtocolSinkStackable_Vtbl {
        unsafe extern "system" fn SwitchSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poiprotsink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SwitchSink(::core::mem::transmute(&poiprotsink)).into()
        }
        unsafe extern "system" fn CommitSwitch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitSwitch().into()
        }
        unsafe extern "system" fn RollbackSwitch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RollbackSwitch().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SwitchSink: SwitchSink::<Identity, Impl, OFFSET>,
            CommitSwitch: CommitSwitch::<Identity, Impl, OFFSET>,
            RollbackSwitch: RollbackSwitch::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolSinkStackable as ::windows::core::Interface>::IID
    }
}
pub trait IInternetSecurityManager_Impl: Sized {
    fn SetSecuritySite(&self, psite: &::core::option::Option<IInternetSecurityMgrSite>) -> ::windows::core::Result<()>;
    fn GetSecuritySite(&self) -> ::windows::core::Result<IInternetSecurityMgrSite>;
    fn MapUrlToZone(&self, pwszurl: &::windows::core::PCWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetSecurityId(&self, pwszurl: &::windows::core::PCWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
    fn ProcessUrlAction(&self, pwszurl: &::windows::core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn QueryCustomPolicy(&self, pwszurl: &::windows::core::PCWSTR, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SetZoneMapping(&self, dwzone: u32, lpszpattern: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternetSecurityManager {}
impl IInternetSecurityManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: isize>() -> IInternetSecurityManager_Vtbl {
        unsafe extern "system" fn SetSecuritySite<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecuritySite(::core::mem::transmute(&psite)).into()
        }
        unsafe extern "system" fn GetSecuritySite<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecuritySite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsite, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapUrlToZone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MapUrlToZone(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&pdwzone), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetSecurityId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSecurityId(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn ProcessUrlAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessUrlAction(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryCustomPolicy(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetZoneMapping<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, lpszpattern: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetZoneMapping(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute(&lpszpattern), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneMappings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, ppenumstring: *mut *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetZoneMappings(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&ppenumstring), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetSecuritySite: SetSecuritySite::<Identity, Impl, OFFSET>,
            GetSecuritySite: GetSecuritySite::<Identity, Impl, OFFSET>,
            MapUrlToZone: MapUrlToZone::<Identity, Impl, OFFSET>,
            GetSecurityId: GetSecurityId::<Identity, Impl, OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Identity, Impl, OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Identity, Impl, OFFSET>,
            SetZoneMapping: SetZoneMapping::<Identity, Impl, OFFSET>,
            GetZoneMappings: GetZoneMappings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSecurityManager as ::windows::core::Interface>::IID
    }
}
pub trait IInternetSecurityManagerEx_Impl: Sized + IInternetSecurityManager_Impl {
    fn ProcessUrlActionEx(&self, pwszurl: &::windows::core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternetSecurityManagerEx {}
impl IInternetSecurityManagerEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManagerEx_Impl, const OFFSET: isize>() -> IInternetSecurityManagerEx_Vtbl {
        unsafe extern "system" fn ProcessUrlActionEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManagerEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessUrlActionEx(::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        Self { base__: IInternetSecurityManager_Vtbl::new::<Identity, Impl, OFFSET>(), ProcessUrlActionEx: ProcessUrlActionEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSecurityManagerEx as ::windows::core::Interface>::IID || iid == &<IInternetSecurityManager as ::windows::core::Interface>::IID
    }
}
pub trait IInternetSecurityManagerEx2_Impl: Sized + IInternetSecurityManager_Impl + IInternetSecurityManagerEx_Impl {
    fn MapUrlToZoneEx2(&self, puri: &::core::option::Option<super::IUri>, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut ::windows::core::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
    fn ProcessUrlActionEx2(&self, puri: &::core::option::Option<super::IUri>, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetSecurityIdEx2(&self, puri: &::core::option::Option<super::IUri>, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
    fn QueryCustomPolicyEx2(&self, puri: &::core::option::Option<super::IUri>, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternetSecurityManagerEx2 {}
impl IInternetSecurityManagerEx2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>() -> IInternetSecurityManagerEx2_Vtbl {
        unsafe extern "system" fn MapUrlToZoneEx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut ::windows::core::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MapUrlToZoneEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&pdwzone), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppwszmappedurl), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn ProcessUrlActionEx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessUrlActionEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn GetSecurityIdEx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSecurityIdEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicyEx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryCustomPolicyEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: IInternetSecurityManagerEx_Vtbl::new::<Identity, Impl, OFFSET>(),
            MapUrlToZoneEx2: MapUrlToZoneEx2::<Identity, Impl, OFFSET>,
            ProcessUrlActionEx2: ProcessUrlActionEx2::<Identity, Impl, OFFSET>,
            GetSecurityIdEx2: GetSecurityIdEx2::<Identity, Impl, OFFSET>,
            QueryCustomPolicyEx2: QueryCustomPolicyEx2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSecurityManagerEx2 as ::windows::core::Interface>::IID || iid == &<IInternetSecurityManager as ::windows::core::Interface>::IID || iid == &<IInternetSecurityManagerEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetSecurityMgrSite_Impl: Sized {
    fn GetWindow(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
    fn EnableModeless(&self, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInternetSecurityMgrSite {}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSecurityMgrSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityMgrSite_Impl, const OFFSET: isize>() -> IInternetSecurityMgrSite_Vtbl {
        unsafe extern "system" fn GetWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityMgrSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSecurityMgrSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableModeless(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSecurityMgrSite as ::windows::core::Interface>::IID
    }
}
pub trait IInternetSession_Impl: Sized {
    fn RegisterNameSpace(&self, pcf: &::core::option::Option<super::IClassFactory>, rclsid: *const ::windows::core::GUID, pwzprotocol: &::windows::core::PCWSTR, cpatterns: u32, ppwzpatterns: *const ::windows::core::PWSTR, dwreserved: u32) -> ::windows::core::Result<()>;
    fn UnregisterNameSpace(&self, pcf: &::core::option::Option<super::IClassFactory>, pszprotocol: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RegisterMimeFilter(&self, pcf: &::core::option::Option<super::IClassFactory>, rclsid: *const ::windows::core::GUID, pwztype: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn UnregisterMimeFilter(&self, pcf: &::core::option::Option<super::IClassFactory>, pwztype: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn CreateBinding(&self, pbc: &::core::option::Option<super::IBindCtx>, szurl: &::windows::core::PCWSTR, punkouter: &::core::option::Option<::windows::core::IUnknown>, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>, ppoinetprot: *mut ::core::option::Option<IInternetProtocol>, dwoption: u32) -> ::windows::core::Result<()>;
    fn SetSessionOption(&self, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetSessionOption(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternetSession {}
impl IInternetSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: isize>() -> IInternetSession_Vtbl {
        unsafe extern "system" fn RegisterNameSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pwzprotocol: ::windows::core::PCWSTR, cpatterns: u32, ppwzpatterns: *const ::windows::core::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterNameSpace(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute(&pwzprotocol), ::core::mem::transmute_copy(&cpatterns), ::core::mem::transmute_copy(&ppwzpatterns), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn UnregisterNameSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, pszprotocol: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterNameSpace(::core::mem::transmute(&pcf), ::core::mem::transmute(&pszprotocol)).into()
        }
        unsafe extern "system" fn RegisterMimeFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, pwztype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterMimeFilter(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute(&pwztype)).into()
        }
        unsafe extern "system" fn UnregisterMimeFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: *mut ::core::ffi::c_void, pwztype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterMimeFilter(::core::mem::transmute(&pcf), ::core::mem::transmute(&pwztype)).into()
        }
        unsafe extern "system" fn CreateBinding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, szurl: ::windows::core::PCWSTR, punkouter: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void, ppoinetprot: *mut *mut ::core::ffi::c_void, dwoption: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateBinding(::core::mem::transmute(&pbc), ::core::mem::transmute(&szurl), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&ppunk), ::core::mem::transmute_copy(&ppoinetprot), ::core::mem::transmute_copy(&dwoption)).into()
        }
        unsafe extern "system" fn SetSessionOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSessionOption(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbufferlength), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetSessionOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSessionOption(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pdwbufferlength), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterNameSpace: RegisterNameSpace::<Identity, Impl, OFFSET>,
            UnregisterNameSpace: UnregisterNameSpace::<Identity, Impl, OFFSET>,
            RegisterMimeFilter: RegisterMimeFilter::<Identity, Impl, OFFSET>,
            UnregisterMimeFilter: UnregisterMimeFilter::<Identity, Impl, OFFSET>,
            CreateBinding: CreateBinding::<Identity, Impl, OFFSET>,
            SetSessionOption: SetSessionOption::<Identity, Impl, OFFSET>,
            GetSessionOption: GetSessionOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSession as ::windows::core::Interface>::IID
    }
}
pub trait IInternetThreadSwitch_Impl: Sized {
    fn Prepare(&self) -> ::windows::core::Result<()>;
    fn Continue(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternetThreadSwitch {}
impl IInternetThreadSwitch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetThreadSwitch_Impl, const OFFSET: isize>() -> IInternetThreadSwitch_Vtbl {
        unsafe extern "system" fn Prepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetThreadSwitch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Prepare().into()
        }
        unsafe extern "system" fn Continue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetThreadSwitch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Continue().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Prepare: Prepare::<Identity, Impl, OFFSET>,
            Continue: Continue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetThreadSwitch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManager_Impl: Sized {
    fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()>;
    fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()>;
    fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()>;
    fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()>;
    fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()>;
    fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()>;
    fn PromptAction(&self, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: &::windows::core::PCWSTR, pwsztext: &::windows::core::PCWSTR, dwpromptflags: u32) -> ::windows::core::Result<()>;
    fn LogAction(&self, dwaction: u32, pwszurl: &::windows::core::PCWSTR, pwsztext: &::windows::core::PCWSTR, dwlogflags: u32) -> ::windows::core::Result<()>;
    fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32>;
    fn DestroyZoneEnumerator(&self, dwenum: u32) -> ::windows::core::Result<()>;
    fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInternetZoneManager {}
#[cfg(feature = "Win32_Foundation")]
impl IInternetZoneManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>() -> IInternetZoneManager_Vtbl {
        unsafe extern "system" fn GetZoneAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetZoneAttributes(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes)).into()
        }
        unsafe extern "system" fn SetZoneAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetZoneAttributes(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes)).into()
        }
        unsafe extern "system" fn GetZoneCustomPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetZoneCustomPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn SetZoneCustomPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetZoneCustomPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn GetZoneActionPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetZoneActionPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn SetZoneActionPolicy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetZoneActionPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn PromptAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR, dwpromptflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PromptAction(::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwsztext), ::core::mem::transmute_copy(&dwpromptflags)).into()
        }
        unsafe extern "system" fn LogAction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, pwszurl: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR, dwlogflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogAction(::core::mem::transmute_copy(&dwaction), ::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwsztext), ::core::mem::transmute_copy(&dwlogflags)).into()
        }
        unsafe extern "system" fn CreateZoneEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateZoneEnumerator(::core::mem::transmute_copy(&pdwenum), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneAt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetZoneAt(::core::mem::transmute_copy(&dwenum), ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwzone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyZoneEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenum: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyZoneEnumerator(::core::mem::transmute_copy(&dwenum)).into()
        }
        unsafe extern "system" fn CopyTemplatePoliciesToZone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyTemplatePoliciesToZone(::core::mem::transmute_copy(&dwtemplate), ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetZoneAttributes: GetZoneAttributes::<Identity, Impl, OFFSET>,
            SetZoneAttributes: SetZoneAttributes::<Identity, Impl, OFFSET>,
            GetZoneCustomPolicy: GetZoneCustomPolicy::<Identity, Impl, OFFSET>,
            SetZoneCustomPolicy: SetZoneCustomPolicy::<Identity, Impl, OFFSET>,
            GetZoneActionPolicy: GetZoneActionPolicy::<Identity, Impl, OFFSET>,
            SetZoneActionPolicy: SetZoneActionPolicy::<Identity, Impl, OFFSET>,
            PromptAction: PromptAction::<Identity, Impl, OFFSET>,
            LogAction: LogAction::<Identity, Impl, OFFSET>,
            CreateZoneEnumerator: CreateZoneEnumerator::<Identity, Impl, OFFSET>,
            GetZoneAt: GetZoneAt::<Identity, Impl, OFFSET>,
            DestroyZoneEnumerator: DestroyZoneEnumerator::<Identity, Impl, OFFSET>,
            CopyTemplatePoliciesToZone: CopyTemplatePoliciesToZone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetZoneManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManagerEx_Impl: Sized + IInternetZoneManager_Impl {
    fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInternetZoneManagerEx {}
#[cfg(feature = "Win32_Foundation")]
impl IInternetZoneManagerEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManagerEx_Impl, const OFFSET: isize>() -> IInternetZoneManagerEx_Vtbl {
        unsafe extern "system" fn GetZoneActionPolicyEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManagerEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetZoneActionPolicyEx(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetZoneActionPolicyEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManagerEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetZoneActionPolicyEx(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: IInternetZoneManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetZoneActionPolicyEx: GetZoneActionPolicyEx::<Identity, Impl, OFFSET>,
            SetZoneActionPolicyEx: SetZoneActionPolicyEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetZoneManagerEx as ::windows::core::Interface>::IID || iid == &<IInternetZoneManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManagerEx2_Impl: Sized + IInternetZoneManager_Impl + IInternetZoneManagerEx_Impl {
    fn GetZoneAttributesEx(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetZoneSecurityState(&self, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetIESecurityState(&self, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FixUnsecureSettings(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInternetZoneManagerEx2 {}
#[cfg(feature = "Win32_Foundation")]
impl IInternetZoneManagerEx2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: isize>() -> IInternetZoneManagerEx2_Vtbl {
        unsafe extern "system" fn GetZoneAttributesEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetZoneAttributesEx(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneSecurityState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetZoneSecurityState(::core::mem::transmute_copy(&dwzoneindex), ::core::mem::transmute_copy(&frespectpolicy), ::core::mem::transmute_copy(&pdwstate), ::core::mem::transmute_copy(&pfpolicyencountered)).into()
        }
        unsafe extern "system" fn GetIESecurityState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIESecurityState(::core::mem::transmute_copy(&frespectpolicy), ::core::mem::transmute_copy(&pdwstate), ::core::mem::transmute_copy(&pfpolicyencountered), ::core::mem::transmute_copy(&fnocache)).into()
        }
        unsafe extern "system" fn FixUnsecureSettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FixUnsecureSettings().into()
        }
        Self {
            base__: IInternetZoneManagerEx_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetZoneAttributesEx: GetZoneAttributesEx::<Identity, Impl, OFFSET>,
            GetZoneSecurityState: GetZoneSecurityState::<Identity, Impl, OFFSET>,
            GetIESecurityState: GetIESecurityState::<Identity, Impl, OFFSET>,
            FixUnsecureSettings: FixUnsecureSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetZoneManagerEx2 as ::windows::core::Interface>::IID || iid == &<IInternetZoneManager as ::windows::core::Interface>::IID || iid == &<IInternetZoneManagerEx as ::windows::core::Interface>::IID
    }
}
pub trait IMonikerProp_Impl: Sized {
    fn PutProperty(&self, mkp: MONIKERPROPERTY, val: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMonikerProp {}
impl IMonikerProp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonikerProp_Impl, const OFFSET: isize>() -> IMonikerProp_Vtbl {
        unsafe extern "system" fn PutProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMonikerProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mkp: MONIKERPROPERTY, val: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutProperty(::core::mem::transmute_copy(&mkp), ::core::mem::transmute(&val)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), PutProperty: PutProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMonikerProp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistMoniker_Impl: Sized {
    fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsDirty(&self) -> ::windows::core::HRESULT;
    fn Load(&self, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: &::core::option::Option<super::IMoniker>, pibc: &::core::option::Option<super::IBindCtx>, grfmode: u32) -> ::windows::core::Result<()>;
    fn Save(&self, pimkname: &::core::option::Option<super::IMoniker>, pbc: &::core::option::Option<super::IBindCtx>, fremember: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SaveCompleted(&self, pimkname: &::core::option::Option<super::IMoniker>, pibc: &::core::option::Option<super::IBindCtx>) -> ::windows::core::Result<()>;
    fn GetCurMoniker(&self) -> ::windows::core::Result<super::IMoniker>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPersistMoniker {}
#[cfg(feature = "Win32_Foundation")]
impl IPersistMoniker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: isize>() -> IPersistMoniker_Vtbl {
        unsafe extern "system" fn GetClassID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClassID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclassid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDirty()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: *mut ::core::ffi::c_void, pibc: *mut ::core::ffi::c_void, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute_copy(&ffullyavailable), ::core::mem::transmute(&pimkname), ::core::mem::transmute(&pibc), ::core::mem::transmute_copy(&grfmode)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimkname: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, fremember: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute(&pimkname), ::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&fremember)).into()
        }
        unsafe extern "system" fn SaveCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimkname: *mut ::core::ffi::c_void, pibc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveCompleted(::core::mem::transmute(&pimkname), ::core::mem::transmute(&pibc)).into()
        }
        unsafe extern "system" fn GetCurMoniker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimkname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurMoniker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimkname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClassID: GetClassID::<Identity, Impl, OFFSET>,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, Impl, OFFSET>,
            GetCurMoniker: GetCurMoniker::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistMoniker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Data_Xml_MsXml")]
pub trait ISoftDistExt_Impl: Sized {
    fn ProcessSoftDist(&self, szcdfurl: &::windows::core::PCWSTR, psoftdistelement: &::core::option::Option<super::super::super::Data::Xml::MsXml::IXMLElement>, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::Result<()>;
    fn GetFirstCodeBase(&self, szcodebase: *const ::windows::core::PWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()>;
    fn GetNextCodeBase(&self, szcodebase: *const ::windows::core::PWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()>;
    fn AsyncInstallDistributionUnit(&self, pbc: &::core::option::Option<super::IBindCtx>, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Data_Xml_MsXml")]
impl ::windows::core::RuntimeName for ISoftDistExt {}
#[cfg(feature = "Win32_Data_Xml_MsXml")]
impl ISoftDistExt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: isize>() -> ISoftDistExt_Vtbl {
        unsafe extern "system" fn ProcessSoftDist<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcdfurl: ::windows::core::PCWSTR, psoftdistelement: *mut ::core::ffi::c_void, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessSoftDist(::core::mem::transmute(&szcdfurl), ::core::mem::transmute(&psoftdistelement), ::core::mem::transmute_copy(&lpsdi)).into()
        }
        unsafe extern "system" fn GetFirstCodeBase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcodebase: *const ::windows::core::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFirstCodeBase(::core::mem::transmute_copy(&szcodebase), ::core::mem::transmute_copy(&dwmaxsize)).into()
        }
        unsafe extern "system" fn GetNextCodeBase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcodebase: *const ::windows::core::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextCodeBase(::core::mem::transmute_copy(&szcodebase), ::core::mem::transmute_copy(&dwmaxsize)).into()
        }
        unsafe extern "system" fn AsyncInstallDistributionUnit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncInstallDistributionUnit(::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&lpcbh)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProcessSoftDist: ProcessSoftDist::<Identity, Impl, OFFSET>,
            GetFirstCodeBase: GetFirstCodeBase::<Identity, Impl, OFFSET>,
            GetNextCodeBase: GetNextCodeBase::<Identity, Impl, OFFSET>,
            AsyncInstallDistributionUnit: AsyncInstallDistributionUnit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISoftDistExt as ::windows::core::Interface>::IID
    }
}
pub trait IUriBuilderFactory_Impl: Sized {
    fn CreateIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder>;
    fn CreateInitializedIUriBuilder(&self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder>;
}
impl ::windows::core::RuntimeName for IUriBuilderFactory {}
impl IUriBuilderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilderFactory_Impl, const OFFSET: isize>() -> IUriBuilderFactory_Vtbl {
        unsafe extern "system" fn CreateIUriBuilder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateIUriBuilder(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuribuilder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInitializedIUriBuilder<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInitializedIUriBuilder(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuribuilder, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateIUriBuilder: CreateIUriBuilder::<Identity, Impl, OFFSET>,
            CreateInitializedIUriBuilder: CreateInitializedIUriBuilder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriBuilderFactory as ::windows::core::Interface>::IID
    }
}
pub trait IUriContainer_Impl: Sized {
    fn GetIUri(&self) -> ::windows::core::Result<super::IUri>;
}
impl ::windows::core::RuntimeName for IUriContainer {}
impl IUriContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriContainer_Impl, const OFFSET: isize>() -> IUriContainer_Vtbl {
        unsafe extern "system" fn GetIUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetIUri: GetIUri::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriContainer as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetCacheHints_Impl: Sized {
    fn SetCacheExtension(&self, pwzext: &::windows::core::PCWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWinInetCacheHints {}
impl IWinInetCacheHints_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetCacheHints_Impl, const OFFSET: isize>() -> IWinInetCacheHints_Vtbl {
        unsafe extern "system" fn SetCacheExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetCacheHints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzext: ::windows::core::PCWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCacheExtension(::core::mem::transmute(&pwzext), ::core::mem::transmute_copy(&pszcachefile), ::core::mem::transmute_copy(&pcbcachefile), ::core::mem::transmute_copy(&pdwwinineterror), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetCacheExtension: SetCacheExtension::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetCacheHints as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetCacheHints2_Impl: Sized + IWinInetCacheHints_Impl {
    fn SetCacheExtension2(&self, pwzext: &::windows::core::PCWSTR, pwzcachefile: ::windows::core::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWinInetCacheHints2 {}
impl IWinInetCacheHints2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetCacheHints2_Impl, const OFFSET: isize>() -> IWinInetCacheHints2_Vtbl {
        unsafe extern "system" fn SetCacheExtension2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetCacheHints2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzext: ::windows::core::PCWSTR, pwzcachefile: ::windows::core::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCacheExtension2(::core::mem::transmute(&pwzext), ::core::mem::transmute_copy(&pwzcachefile), ::core::mem::transmute_copy(&pcchcachefile), ::core::mem::transmute_copy(&pdwwinineterror), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: IWinInetCacheHints_Vtbl::new::<Identity, Impl, OFFSET>(), SetCacheExtension2: SetCacheExtension2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetCacheHints2 as ::windows::core::Interface>::IID || iid == &<IWinInetCacheHints as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetFileStream_Impl: Sized {
    fn SetHandleForUnlock(&self, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::Result<()>;
    fn SetDeleteFile(&self, dwreserved: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWinInetFileStream {}
impl IWinInetFileStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetFileStream_Impl, const OFFSET: isize>() -> IWinInetFileStream_Vtbl {
        unsafe extern "system" fn SetHandleForUnlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHandleForUnlock(::core::mem::transmute_copy(&hwininetlockhandle), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetDeleteFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeleteFile(::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetHandleForUnlock: SetHandleForUnlock::<Identity, Impl, OFFSET>,
            SetDeleteFile: SetDeleteFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetFileStream as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetHttpInfo_Impl: Sized + IWinInetInfo_Impl {
    fn QueryInfo(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWinInetHttpInfo {}
impl IWinInetHttpInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetHttpInfo_Impl, const OFFSET: isize>() -> IWinInetHttpInfo_Vtbl {
        unsafe extern "system" fn QueryInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetHttpInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryInfo(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbbuf), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: IWinInetInfo_Vtbl::new::<Identity, Impl, OFFSET>(), QueryInfo: QueryInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetHttpInfo as ::windows::core::Interface>::IID || iid == &<IWinInetInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetHttpTimeouts_Impl: Sized {
    fn GetRequestTimeouts(&self, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWinInetHttpTimeouts {}
impl IWinInetHttpTimeouts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetHttpTimeouts_Impl, const OFFSET: isize>() -> IWinInetHttpTimeouts_Vtbl {
        unsafe extern "system" fn GetRequestTimeouts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetHttpTimeouts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRequestTimeouts(::core::mem::transmute_copy(&pdwconnecttimeout), ::core::mem::transmute_copy(&pdwsendtimeout), ::core::mem::transmute_copy(&pdwreceivetimeout)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetRequestTimeouts: GetRequestTimeouts::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetHttpTimeouts as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetInfo_Impl: Sized {
    fn QueryOption(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWinInetInfo {}
impl IWinInetInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetInfo_Impl, const OFFSET: isize>() -> IWinInetInfo_Vtbl {
        unsafe extern "system" fn QueryOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWinInetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryOption(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbbuf)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), QueryOption: QueryOption::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowForBindingUI_Impl: Sized {
    fn GetWindow(&self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWindowForBindingUI {}
#[cfg(feature = "Win32_Foundation")]
impl IWindowForBindingUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWindowForBindingUI_Impl, const OFFSET: isize>() -> IWindowForBindingUI_Vtbl {
        unsafe extern "system" fn GetWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWindowForBindingUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidreason: *const ::windows::core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWindow(::core::mem::transmute_copy(&rguidreason)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetWindow: GetWindow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowForBindingUI as ::windows::core::Interface>::IID
    }
}
pub trait IWrappedProtocol_Impl: Sized {
    fn GetWrapperCode(&self, pncode: *mut i32, dwreserved: usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWrappedProtocol {}
impl IWrappedProtocol_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWrappedProtocol_Impl, const OFFSET: isize>() -> IWrappedProtocol_Vtbl {
        unsafe extern "system" fn GetWrapperCode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWrappedProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncode: *mut i32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWrapperCode(::core::mem::transmute_copy(&pncode), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetWrapperCode: GetWrapperCode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWrappedProtocol as ::windows::core::Interface>::IID
    }
}
pub trait IZoneIdentifier_Impl: Sized {
    fn GetId(&self) -> ::windows::core::Result<u32>;
    fn SetId(&self, dwzone: u32) -> ::windows::core::Result<()>;
    fn Remove(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IZoneIdentifier {}
impl IZoneIdentifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier_Impl, const OFFSET: isize>() -> IZoneIdentifier_Vtbl {
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwzone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute_copy(&dwzone)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoneIdentifier as ::windows::core::Interface>::IID
    }
}
pub trait IZoneIdentifier2_Impl: Sized + IZoneIdentifier_Impl {
    fn GetLastWriterPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetLastWriterPackageFamilyName(&self, packagefamilyname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RemoveLastWriterPackageFamilyName(&self) -> ::windows::core::Result<()>;
    fn GetAppZoneId(&self) -> ::windows::core::Result<u32>;
    fn SetAppZoneId(&self, zone: u32) -> ::windows::core::Result<()>;
    fn RemoveAppZoneId(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IZoneIdentifier2 {}
impl IZoneIdentifier2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: isize>() -> IZoneIdentifier2_Vtbl {
        unsafe extern "system" fn GetLastWriterPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastWriterPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefamilyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastWriterPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLastWriterPackageFamilyName(::core::mem::transmute(&packagefamilyname)).into()
        }
        unsafe extern "system" fn RemoveLastWriterPackageFamilyName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLastWriterPackageFamilyName().into()
        }
        unsafe extern "system" fn GetAppZoneId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAppZoneId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(zone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppZoneId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zone: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAppZoneId(::core::mem::transmute_copy(&zone)).into()
        }
        unsafe extern "system" fn RemoveAppZoneId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAppZoneId().into()
        }
        Self {
            base__: IZoneIdentifier_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLastWriterPackageFamilyName: GetLastWriterPackageFamilyName::<Identity, Impl, OFFSET>,
            SetLastWriterPackageFamilyName: SetLastWriterPackageFamilyName::<Identity, Impl, OFFSET>,
            RemoveLastWriterPackageFamilyName: RemoveLastWriterPackageFamilyName::<Identity, Impl, OFFSET>,
            GetAppZoneId: GetAppZoneId::<Identity, Impl, OFFSET>,
            SetAppZoneId: SetAppZoneId::<Identity, Impl, OFFSET>,
            RemoveAppZoneId: RemoveAppZoneId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoneIdentifier2 as ::windows::core::Interface>::IID || iid == &<IZoneIdentifier as ::windows::core::Interface>::IID
    }
}
