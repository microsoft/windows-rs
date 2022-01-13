#[cfg(feature = "Win32_Foundation")]
pub trait IBindCallbackRedirectImpl: Sized {
    fn Redirect(&mut self, lpcurl: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<i16>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBindCallbackRedirectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindCallbackRedirectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindCallbackRedirectVtbl {
        unsafe extern "system" fn Redirect<Impl: IBindCallbackRedirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcurl: super::super::super::Foundation::PWSTR, vbcancel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Redirect(::core::mem::transmute_copy(&lpcurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *vbcancel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Redirect: Redirect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindCallbackRedirect as ::windows::core::Interface>::IID
    }
}
pub trait IBindHttpSecurityImpl: Sized {
    fn GetIgnoreCertMask(&mut self) -> ::windows::core::Result<u32>;
}
impl IBindHttpSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindHttpSecurityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindHttpSecurityVtbl {
        unsafe extern "system" fn GetIgnoreCertMask<Impl: IBindHttpSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwignorecertmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIgnoreCertMask() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwignorecertmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIgnoreCertMask: GetIgnoreCertMask::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindHttpSecurity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBindProtocolImpl: Sized {
    fn CreateBinding(&mut self, szurl: super::super::super::Foundation::PWSTR, pbc: ::core::option::Option<super::IBindCtx>) -> ::windows::core::Result<super::IBinding>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBindProtocolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindProtocolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindProtocolVtbl {
        unsafe extern "system" fn CreateBinding<Impl: IBindProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: super::super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, ppb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBinding(::core::mem::transmute_copy(&szurl), ::core::mem::transmute(&pbc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateBinding: CreateBinding::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindProtocol as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICatalogFileInfoImpl: Sized {
    fn GetCatalogFile(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PSTR>;
    fn GetJavaTrust(&mut self, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICatalogFileInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogFileInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICatalogFileInfoVtbl {
        unsafe extern "system" fn GetCatalogFile<Impl: ICatalogFileInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcatalogfile: *mut super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCatalogFile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcatalogfile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJavaTrust<Impl: ICatalogFileInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetJavaTrust(::core::mem::transmute_copy(&ppjavatrust)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCatalogFile: GetCatalogFile::<Impl, IMPL_OFFSET>,
            GetJavaTrust: GetJavaTrust::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalogFileInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICodeInstallImpl: Sized + IWindowForBindingUIImpl {
    fn OnCodeInstallProblem(&mut self, ulstatuscode: u32, szdestination: super::super::super::Foundation::PWSTR, szsource: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICodeInstallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICodeInstallImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICodeInstallVtbl {
        unsafe extern "system" fn OnCodeInstallProblem<Impl: ICodeInstallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szdestination: super::super::super::Foundation::PWSTR, szsource: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCodeInstallProblem(::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute_copy(&szdestination), ::core::mem::transmute_copy(&szsource), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: IWindowForBindingUIVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnCodeInstallProblem: OnCodeInstallProblem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICodeInstall as ::windows::core::Interface>::IID
    }
}
pub trait IDataFilterImpl: Sized {
    fn DoEncode(&mut self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn DoDecode(&mut self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SetEncodingLevel(&mut self, dwenclevel: u32) -> ::windows::core::Result<()>;
}
impl IDataFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataFilterVtbl {
        unsafe extern "system" fn DoEncode<Impl: IDataFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoEncode(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&linbuffersize), ::core::mem::transmute_copy(&pbinbuffer), ::core::mem::transmute_copy(&loutbuffersize), ::core::mem::transmute_copy(&pboutbuffer), ::core::mem::transmute_copy(&linbytesavailable), ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn DoDecode<Impl: IDataFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoDecode(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&linbuffersize), ::core::mem::transmute_copy(&pbinbuffer), ::core::mem::transmute_copy(&loutbuffersize), ::core::mem::transmute_copy(&pboutbuffer), ::core::mem::transmute_copy(&linbytesavailable), ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetEncodingLevel<Impl: IDataFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenclevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEncodingLevel(::core::mem::transmute_copy(&dwenclevel)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DoEncode: DoEncode::<Impl, IMPL_OFFSET>,
            DoDecode: DoDecode::<Impl, IMPL_OFFSET>,
            SetEncodingLevel: SetEncodingLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEncodingFilterFactoryImpl: Sized {
    fn FindBestFilter(&mut self, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, info: DATAINFO) -> ::windows::core::Result<IDataFilter>;
    fn GetDefaultFilter(&mut self, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IDataFilter>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEncodingFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEncodingFilterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEncodingFilterFactoryVtbl {
        unsafe extern "system" fn FindBestFilter<Impl: IEncodingFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, info: DATAINFO, ppdf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindBestFilter(::core::mem::transmute_copy(&pwzcodein), ::core::mem::transmute_copy(&pwzcodeout), ::core::mem::transmute_copy(&info)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFilter<Impl: IEncodingFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, ppdf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultFilter(::core::mem::transmute_copy(&pwzcodein), ::core::mem::transmute_copy(&pwzcodeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FindBestFilter: FindBestFilter::<Impl, IMPL_OFFSET>,
            GetDefaultFilter: GetDefaultFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEncodingFilterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetBindHandleImpl: Sized {
    fn GetBindHandle(&mut self, enumrequestedhandle: BINDHANDLETYPES) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGetBindHandleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetBindHandleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetBindHandleVtbl {
        unsafe extern "system" fn GetBindHandle<Impl: IGetBindHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumrequestedhandle: BINDHANDLETYPES, prethandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindHandle(::core::mem::transmute_copy(&enumrequestedhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *prethandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetBindHandle: GetBindHandle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetBindHandle as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHttpNegotiateImpl: Sized {
    fn BeginningTransaction(&mut self, szurl: super::super::super::Foundation::PWSTR, szheaders: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn OnResponse(&mut self, dwresponsecode: u32, szresponseheaders: super::super::super::Foundation::PWSTR, szrequestheaders: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHttpNegotiateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNegotiateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpNegotiateVtbl {
        unsafe extern "system" fn BeginningTransaction<Impl: IHttpNegotiateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: super::super::super::Foundation::PWSTR, szheaders: super::super::super::Foundation::PWSTR, dwreserved: u32, pszadditionalheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginningTransaction(::core::mem::transmute_copy(&szurl), ::core::mem::transmute_copy(&szheaders), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszadditionalheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResponse<Impl: IHttpNegotiateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwresponsecode: u32, szresponseheaders: super::super::super::Foundation::PWSTR, szrequestheaders: super::super::super::Foundation::PWSTR, pszadditionalrequestheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnResponse(::core::mem::transmute_copy(&dwresponsecode), ::core::mem::transmute_copy(&szresponseheaders), ::core::mem::transmute_copy(&szrequestheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszadditionalrequestheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginningTransaction: BeginningTransaction::<Impl, IMPL_OFFSET>,
            OnResponse: OnResponse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNegotiate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHttpNegotiate2Impl: Sized + IHttpNegotiateImpl {
    fn GetRootSecurityId(&mut self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHttpNegotiate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNegotiate2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpNegotiate2Vtbl {
        unsafe extern "system" fn GetRootSecurityId<Impl: IHttpNegotiate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRootSecurityId(::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base: IHttpNegotiateVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetRootSecurityId: GetRootSecurityId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNegotiate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHttpNegotiate3Impl: Sized + IHttpNegotiateImpl + IHttpNegotiate2Impl {
    fn GetSerializedClientCertContext(&mut self, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHttpNegotiate3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNegotiate3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpNegotiate3Vtbl {
        unsafe extern "system" fn GetSerializedClientCertContext<Impl: IHttpNegotiate3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSerializedClientCertContext(::core::mem::transmute_copy(&ppbcert), ::core::mem::transmute_copy(&pcbcert)).into()
        }
        Self {
            base: IHttpNegotiate2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSerializedClientCertContext: GetSerializedClientCertContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNegotiate3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHttpSecurityImpl: Sized + IWindowForBindingUIImpl {
    fn OnSecurityProblem(&mut self, dwproblem: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHttpSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpSecurityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpSecurityVtbl {
        unsafe extern "system" fn OnSecurityProblem<Impl: IHttpSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproblem: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSecurityProblem(::core::mem::transmute_copy(&dwproblem)).into()
        }
        Self { base: IWindowForBindingUIVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnSecurityProblem: OnSecurityProblem::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpSecurity as ::windows::core::Interface>::IID
    }
}
pub trait IInternetImpl: Sized {}
impl IInternetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetVtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInternetBindInfoImpl: Sized {
    fn GetBindInfo(&mut self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::Result<()>;
    fn GetBindString(&mut self, ulstringtype: u32, ppwzstr: *mut super::super::super::Foundation::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IInternetBindInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetBindInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetBindInfoVtbl {
        unsafe extern "system" fn GetBindInfo<Impl: IInternetBindInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBindInfo(::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo)).into()
        }
        unsafe extern "system" fn GetBindString<Impl: IInternetBindInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstringtype: u32, ppwzstr: *mut super::super::super::Foundation::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBindString(::core::mem::transmute_copy(&ulstringtype), ::core::mem::transmute_copy(&ppwzstr), ::core::mem::transmute_copy(&cel), ::core::mem::transmute_copy(&pcelfetched)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBindInfo: GetBindInfo::<Impl, IMPL_OFFSET>,
            GetBindString: GetBindString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetBindInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInternetBindInfoExImpl: Sized + IInternetBindInfoImpl {
    fn GetBindInfoEx(&mut self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IInternetBindInfoExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetBindInfoExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetBindInfoExVtbl {
        unsafe extern "system" fn GetBindInfoEx<Impl: IInternetBindInfoExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBindInfoEx(::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&grfbindf2), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base: IInternetBindInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetBindInfoEx: GetBindInfoEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetBindInfoEx as ::windows::core::Interface>::IID
    }
}
pub trait IInternetHostSecurityManagerImpl: Sized {
    fn GetSecurityId(&mut self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
    fn ProcessUrlAction(&mut self, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn QueryCustomPolicy(&mut self, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl IInternetHostSecurityManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetHostSecurityManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetHostSecurityManagerVtbl {
        unsafe extern "system" fn GetSecurityId<Impl: IInternetHostSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSecurityId(::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn ProcessUrlAction<Impl: IInternetHostSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUrlAction(::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicy<Impl: IInternetHostSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryCustomPolicy(::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSecurityId: GetSecurityId::<Impl, IMPL_OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Impl, IMPL_OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetHostSecurityManager as ::windows::core::Interface>::IID
    }
}
pub trait IInternetPriorityImpl: Sized {
    fn SetPriority(&mut self, npriority: i32) -> ::windows::core::Result<()>;
    fn GetPriority(&mut self) -> ::windows::core::Result<i32>;
}
impl IInternetPriorityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetPriorityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetPriorityVtbl {
        unsafe extern "system" fn SetPriority<Impl: IInternetPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&npriority)).into()
        }
        unsafe extern "system" fn GetPriority<Impl: IInternetPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *pnpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            GetPriority: GetPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetPriority as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolImpl: Sized + IInternetProtocolRootImpl {
    fn Read(&mut self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()>;
    fn Seek(&mut self, dlibmove: i64, dworigin: u32) -> ::windows::core::Result<u64>;
    fn LockRequest(&mut self, dwoptions: u32) -> ::windows::core::Result<()>;
    fn UnlockRequest(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolVtbl {
        unsafe extern "system" fn Read<Impl: IInternetProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn Seek<Impl: IInternetProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&dworigin)) {
                ::core::result::Result::Ok(ok__) => {
                    *plibnewposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRequest<Impl: IInternetProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockRequest(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn UnlockRequest<Impl: IInternetProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnlockRequest().into()
        }
        Self {
            base: IInternetProtocolRootVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Read: Read::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
            LockRequest: LockRequest::<Impl, IMPL_OFFSET>,
            UnlockRequest: UnlockRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocol as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolExImpl: Sized + IInternetProtocolRootImpl + IInternetProtocolImpl {
    fn StartEx(&mut self, puri: ::core::option::Option<super::IUri>, poiprotsink: ::core::option::Option<IInternetProtocolSink>, poibindinfo: ::core::option::Option<IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolExVtbl {
        unsafe extern "system" fn StartEx<Impl: IInternetProtocolExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartEx(::core::mem::transmute(&puri), ::core::mem::transmute(&poiprotsink), ::core::mem::transmute(&poibindinfo), ::core::mem::transmute_copy(&grfpi), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base: IInternetProtocolVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), StartEx: StartEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolInfoImpl: Sized {
    fn ParseUrl(&mut self, pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn CombineUrl(&mut self, pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn CompareUrl(&mut self, pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwcompareflags: u32) -> ::windows::core::Result<()>;
    fn QueryInfo(&mut self, pwzurl: super::super::super::Foundation::PWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolInfoVtbl {
        unsafe extern "system" fn ParseUrl<Impl: IInternetProtocolInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseUrl(::core::mem::transmute_copy(&pwzurl), ::core::mem::transmute_copy(&parseaction), ::core::mem::transmute_copy(&dwparseflags), ::core::mem::transmute_copy(&pwzresult), ::core::mem::transmute_copy(&cchresult), ::core::mem::transmute_copy(&pcchresult), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn CombineUrl<Impl: IInternetProtocolInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CombineUrl(::core::mem::transmute_copy(&pwzbaseurl), ::core::mem::transmute_copy(&pwzrelativeurl), ::core::mem::transmute_copy(&dwcombineflags), ::core::mem::transmute_copy(&pwzresult), ::core::mem::transmute_copy(&cchresult), ::core::mem::transmute_copy(&pcchresult), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn CompareUrl<Impl: IInternetProtocolInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwcompareflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompareUrl(::core::mem::transmute_copy(&pwzurl1), ::core::mem::transmute_copy(&pwzurl2), ::core::mem::transmute_copy(&dwcompareflags)).into()
        }
        unsafe extern "system" fn QueryInfo<Impl: IInternetProtocolInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl: super::super::super::Foundation::PWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInfo(::core::mem::transmute_copy(&pwzurl), ::core::mem::transmute_copy(&oueryoption), ::core::mem::transmute_copy(&dwqueryflags), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcbbuf), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ParseUrl: ParseUrl::<Impl, IMPL_OFFSET>,
            CombineUrl: CombineUrl::<Impl, IMPL_OFFSET>,
            CompareUrl: CompareUrl::<Impl, IMPL_OFFSET>,
            QueryInfo: QueryInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolRootImpl: Sized {
    fn Start(&mut self, szurl: super::super::super::Foundation::PWSTR, poiprotsink: ::core::option::Option<IInternetProtocolSink>, poibindinfo: ::core::option::Option<IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn Continue(&mut self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()>;
    fn Abort(&mut self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()>;
    fn Terminate(&mut self, dwoptions: u32) -> ::windows::core::Result<()>;
    fn Suspend(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolRootVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolRootImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolRootVtbl {
        unsafe extern "system" fn Start<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: super::super::super::Foundation::PWSTR, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&szurl), ::core::mem::transmute(&poiprotsink), ::core::mem::transmute(&poibindinfo), ::core::mem::transmute_copy(&grfpi), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Continue<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Continue(::core::mem::transmute_copy(&pprotocoldata)).into()
        }
        unsafe extern "system" fn Abort<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort(::core::mem::transmute_copy(&hrreason), ::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn Terminate<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn Suspend<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Suspend().into()
        }
        unsafe extern "system" fn Resume<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Continue: Continue::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
            Suspend: Suspend::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolRoot as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolSinkImpl: Sized {
    fn Switch(&mut self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()>;
    fn ReportProgress(&mut self, ulstatuscode: u32, szstatustext: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ReportData(&mut self, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()>;
    fn ReportResult(&mut self, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolSinkVtbl {
        unsafe extern "system" fn Switch<Impl: IInternetProtocolSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Switch(::core::mem::transmute_copy(&pprotocoldata)).into()
        }
        unsafe extern "system" fn ReportProgress<Impl: IInternetProtocolSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szstatustext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportProgress(::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute_copy(&szstatustext)).into()
        }
        unsafe extern "system" fn ReportData<Impl: IInternetProtocolSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportData(::core::mem::transmute_copy(&grfbscf), ::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax)).into()
        }
        unsafe extern "system" fn ReportResult<Impl: IInternetProtocolSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportResult(::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&dwerror), ::core::mem::transmute_copy(&szresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Switch: Switch::<Impl, IMPL_OFFSET>,
            ReportProgress: ReportProgress::<Impl, IMPL_OFFSET>,
            ReportData: ReportData::<Impl, IMPL_OFFSET>,
            ReportResult: ReportResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolSink as ::windows::core::Interface>::IID
    }
}
pub trait IInternetProtocolSinkStackableImpl: Sized {
    fn SwitchSink(&mut self, poiprotsink: ::core::option::Option<IInternetProtocolSink>) -> ::windows::core::Result<()>;
    fn CommitSwitch(&mut self) -> ::windows::core::Result<()>;
    fn RollbackSwitch(&mut self) -> ::windows::core::Result<()>;
}
impl IInternetProtocolSinkStackableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolSinkStackableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolSinkStackableVtbl {
        unsafe extern "system" fn SwitchSink<Impl: IInternetProtocolSinkStackableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poiprotsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwitchSink(::core::mem::transmute(&poiprotsink)).into()
        }
        unsafe extern "system" fn CommitSwitch<Impl: IInternetProtocolSinkStackableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitSwitch().into()
        }
        unsafe extern "system" fn RollbackSwitch<Impl: IInternetProtocolSinkStackableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RollbackSwitch().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SwitchSink: SwitchSink::<Impl, IMPL_OFFSET>,
            CommitSwitch: CommitSwitch::<Impl, IMPL_OFFSET>,
            RollbackSwitch: RollbackSwitch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolSinkStackable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetSecurityManagerImpl: Sized {
    fn SetSecuritySite(&mut self, psite: ::core::option::Option<IInternetSecurityMgrSite>) -> ::windows::core::Result<()>;
    fn GetSecuritySite(&mut self) -> ::windows::core::Result<IInternetSecurityMgrSite>;
    fn MapUrlToZone(&mut self, pwszurl: super::super::super::Foundation::PWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetSecurityId(&mut self, pwszurl: super::super::super::Foundation::PWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
    fn ProcessUrlAction(&mut self, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn QueryCustomPolicy(&mut self, pwszurl: super::super::super::Foundation::PWSTR, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SetZoneMapping(&mut self, dwzone: u32, lpszpattern: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetZoneMappings(&mut self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSecurityManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSecurityManagerVtbl {
        unsafe extern "system" fn SetSecuritySite<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecuritySite(::core::mem::transmute(&psite)).into()
        }
        unsafe extern "system" fn GetSecuritySite<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecuritySite() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapUrlToZone<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MapUrlToZone(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pdwzone), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetSecurityId<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSecurityId(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn ProcessUrlAction<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUrlAction(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicy<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryCustomPolicy(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetZoneMapping<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, lpszpattern: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneMapping(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&lpszpattern), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneMappings<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, ppenumstring: *mut ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneMappings(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&ppenumstring), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetSecuritySite: SetSecuritySite::<Impl, IMPL_OFFSET>,
            GetSecuritySite: GetSecuritySite::<Impl, IMPL_OFFSET>,
            MapUrlToZone: MapUrlToZone::<Impl, IMPL_OFFSET>,
            GetSecurityId: GetSecurityId::<Impl, IMPL_OFFSET>,
            ProcessUrlAction: ProcessUrlAction::<Impl, IMPL_OFFSET>,
            QueryCustomPolicy: QueryCustomPolicy::<Impl, IMPL_OFFSET>,
            SetZoneMapping: SetZoneMapping::<Impl, IMPL_OFFSET>,
            GetZoneMappings: GetZoneMappings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSecurityManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetSecurityManagerExImpl: Sized + IInternetSecurityManagerImpl {
    fn ProcessUrlActionEx(&mut self, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSecurityManagerExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityManagerExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSecurityManagerExVtbl {
        unsafe extern "system" fn ProcessUrlActionEx<Impl: IInternetSecurityManagerExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUrlActionEx(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        Self {
            base: IInternetSecurityManagerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ProcessUrlActionEx: ProcessUrlActionEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSecurityManagerEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetSecurityManagerEx2Impl: Sized + IInternetSecurityManagerImpl + IInternetSecurityManagerExImpl {
    fn MapUrlToZoneEx2(&mut self, puri: ::core::option::Option<super::IUri>, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut super::super::super::Foundation::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
    fn ProcessUrlActionEx2(&mut self, puri: ::core::option::Option<super::IUri>, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetSecurityIdEx2(&mut self, puri: ::core::option::Option<super::IUri>, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
    fn QueryCustomPolicyEx2(&mut self, puri: ::core::option::Option<super::IUri>, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSecurityManagerEx2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityManagerEx2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSecurityManagerEx2Vtbl {
        unsafe extern "system" fn MapUrlToZoneEx2<Impl: IInternetSecurityManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut super::super::super::Foundation::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MapUrlToZoneEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&pdwzone), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppwszmappedurl), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn ProcessUrlActionEx2<Impl: IInternetSecurityManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUrlActionEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn GetSecurityIdEx2<Impl: IInternetSecurityManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSecurityIdEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicyEx2<Impl: IInternetSecurityManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryCustomPolicyEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: IInternetSecurityManagerExVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MapUrlToZoneEx2: MapUrlToZoneEx2::<Impl, IMPL_OFFSET>,
            ProcessUrlActionEx2: ProcessUrlActionEx2::<Impl, IMPL_OFFSET>,
            GetSecurityIdEx2: GetSecurityIdEx2::<Impl, IMPL_OFFSET>,
            QueryCustomPolicyEx2: QueryCustomPolicyEx2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSecurityManagerEx2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetSecurityMgrSiteImpl: Sized {
    fn GetWindow(&mut self) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
    fn EnableModeless(&mut self, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSecurityMgrSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityMgrSiteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSecurityMgrSiteVtbl {
        unsafe extern "system" fn GetWindow<Impl: IInternetSecurityMgrSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableModeless<Impl: IInternetSecurityMgrSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableModeless(::core::mem::transmute_copy(&fenable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetWindow: GetWindow::<Impl, IMPL_OFFSET>,
            EnableModeless: EnableModeless::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSecurityMgrSite as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetSessionImpl: Sized {
    fn RegisterNameSpace(&mut self, pcf: ::core::option::Option<super::IClassFactory>, rclsid: *const ::windows::core::GUID, pwzprotocol: super::super::super::Foundation::PWSTR, cpatterns: u32, ppwzpatterns: *const super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<()>;
    fn UnregisterNameSpace(&mut self, pcf: ::core::option::Option<super::IClassFactory>, pszprotocol: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RegisterMimeFilter(&mut self, pcf: ::core::option::Option<super::IClassFactory>, rclsid: *const ::windows::core::GUID, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn UnregisterMimeFilter(&mut self, pcf: ::core::option::Option<super::IClassFactory>, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CreateBinding(&mut self, pbc: ::core::option::Option<super::IBindCtx>, szurl: super::super::super::Foundation::PWSTR, punkouter: ::core::option::Option<::windows::core::IUnknown>, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>, ppoinetprot: *mut ::core::option::Option<IInternetProtocol>, dwoption: u32) -> ::windows::core::Result<()>;
    fn SetSessionOption(&mut self, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetSessionOption(&mut self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSessionVtbl {
        unsafe extern "system" fn RegisterNameSpace<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, pwzprotocol: super::super::super::Foundation::PWSTR, cpatterns: u32, ppwzpatterns: *const super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterNameSpace(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&pwzprotocol), ::core::mem::transmute_copy(&cpatterns), ::core::mem::transmute_copy(&ppwzpatterns), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn UnregisterNameSpace<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, pszprotocol: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterNameSpace(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&pszprotocol)).into()
        }
        unsafe extern "system" fn RegisterMimeFilter<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterMimeFilter(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&pwztype)).into()
        }
        unsafe extern "system" fn UnregisterMimeFilter<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterMimeFilter(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&pwztype)).into()
        }
        unsafe extern "system" fn CreateBinding<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, punkouter: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void, ppoinetprot: *mut ::windows::core::RawPtr, dwoption: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateBinding(::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&szurl), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&ppunk), ::core::mem::transmute_copy(&ppoinetprot), ::core::mem::transmute_copy(&dwoption)).into()
        }
        unsafe extern "system" fn SetSessionOption<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionOption(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbufferlength), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetSessionOption<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSessionOption(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pdwbufferlength), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterNameSpace: RegisterNameSpace::<Impl, IMPL_OFFSET>,
            UnregisterNameSpace: UnregisterNameSpace::<Impl, IMPL_OFFSET>,
            RegisterMimeFilter: RegisterMimeFilter::<Impl, IMPL_OFFSET>,
            UnregisterMimeFilter: UnregisterMimeFilter::<Impl, IMPL_OFFSET>,
            CreateBinding: CreateBinding::<Impl, IMPL_OFFSET>,
            SetSessionOption: SetSessionOption::<Impl, IMPL_OFFSET>,
            GetSessionOption: GetSessionOption::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSession as ::windows::core::Interface>::IID
    }
}
pub trait IInternetThreadSwitchImpl: Sized {
    fn Prepare(&mut self) -> ::windows::core::Result<()>;
    fn Continue(&mut self) -> ::windows::core::Result<()>;
}
impl IInternetThreadSwitchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetThreadSwitchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetThreadSwitchVtbl {
        unsafe extern "system" fn Prepare<Impl: IInternetThreadSwitchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Prepare().into()
        }
        unsafe extern "system" fn Continue<Impl: IInternetThreadSwitchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Continue().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Prepare: Prepare::<Impl, IMPL_OFFSET>,
            Continue: Continue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetThreadSwitch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManagerImpl: Sized {
    fn GetZoneAttributes(&mut self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()>;
    fn SetZoneAttributes(&mut self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()>;
    fn GetZoneCustomPolicy(&mut self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()>;
    fn SetZoneCustomPolicy(&mut self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()>;
    fn GetZoneActionPolicy(&mut self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()>;
    fn SetZoneActionPolicy(&mut self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()>;
    fn PromptAction(&mut self, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwpromptflags: u32) -> ::windows::core::Result<()>;
    fn LogAction(&mut self, dwaction: u32, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwlogflags: u32) -> ::windows::core::Result<()>;
    fn CreateZoneEnumerator(&mut self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetZoneAt(&mut self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32>;
    fn DestroyZoneEnumerator(&mut self, dwenum: u32) -> ::windows::core::Result<()>;
    fn CopyTemplatePoliciesToZone(&mut self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetZoneManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetZoneManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetZoneManagerVtbl {
        unsafe extern "system" fn GetZoneAttributes<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneAttributes(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes)).into()
        }
        unsafe extern "system" fn SetZoneAttributes<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneAttributes(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes)).into()
        }
        unsafe extern "system" fn GetZoneCustomPolicy<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneCustomPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn SetZoneCustomPolicy<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneCustomPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn GetZoneActionPolicy<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneActionPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn SetZoneActionPolicy<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneActionPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn PromptAction<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwpromptflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PromptAction(::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&dwpromptflags)).into()
        }
        unsafe extern "system" fn LogAction<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwlogflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogAction(::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&dwlogflags)).into()
        }
        unsafe extern "system" fn CreateZoneEnumerator<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateZoneEnumerator(::core::mem::transmute_copy(&pdwenum), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneAt<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneAt(::core::mem::transmute_copy(&dwenum), ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwzone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyZoneEnumerator<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenum: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyZoneEnumerator(::core::mem::transmute_copy(&dwenum)).into()
        }
        unsafe extern "system" fn CopyTemplatePoliciesToZone<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyTemplatePoliciesToZone(::core::mem::transmute_copy(&dwtemplate), ::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetZoneAttributes: GetZoneAttributes::<Impl, IMPL_OFFSET>,
            SetZoneAttributes: SetZoneAttributes::<Impl, IMPL_OFFSET>,
            GetZoneCustomPolicy: GetZoneCustomPolicy::<Impl, IMPL_OFFSET>,
            SetZoneCustomPolicy: SetZoneCustomPolicy::<Impl, IMPL_OFFSET>,
            GetZoneActionPolicy: GetZoneActionPolicy::<Impl, IMPL_OFFSET>,
            SetZoneActionPolicy: SetZoneActionPolicy::<Impl, IMPL_OFFSET>,
            PromptAction: PromptAction::<Impl, IMPL_OFFSET>,
            LogAction: LogAction::<Impl, IMPL_OFFSET>,
            CreateZoneEnumerator: CreateZoneEnumerator::<Impl, IMPL_OFFSET>,
            GetZoneAt: GetZoneAt::<Impl, IMPL_OFFSET>,
            DestroyZoneEnumerator: DestroyZoneEnumerator::<Impl, IMPL_OFFSET>,
            CopyTemplatePoliciesToZone: CopyTemplatePoliciesToZone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetZoneManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManagerExImpl: Sized + IInternetZoneManagerImpl {
    fn GetZoneActionPolicyEx(&mut self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetZoneActionPolicyEx(&mut self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetZoneManagerExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetZoneManagerExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetZoneManagerExVtbl {
        unsafe extern "system" fn GetZoneActionPolicyEx<Impl: IInternetZoneManagerExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneActionPolicyEx(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetZoneActionPolicyEx<Impl: IInternetZoneManagerExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneActionPolicyEx(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: IInternetZoneManagerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetZoneActionPolicyEx: GetZoneActionPolicyEx::<Impl, IMPL_OFFSET>,
            SetZoneActionPolicyEx: SetZoneActionPolicyEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetZoneManagerEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManagerEx2Impl: Sized + IInternetZoneManagerImpl + IInternetZoneManagerExImpl {
    fn GetZoneAttributesEx(&mut self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetZoneSecurityState(&mut self, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetIESecurityState(&mut self, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FixUnsecureSettings(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetZoneManagerEx2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetZoneManagerEx2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetZoneManagerEx2Vtbl {
        unsafe extern "system" fn GetZoneAttributesEx<Impl: IInternetZoneManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneAttributesEx(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneSecurityState<Impl: IInternetZoneManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneSecurityState(::core::mem::transmute_copy(&dwzoneindex), ::core::mem::transmute_copy(&frespectpolicy), ::core::mem::transmute_copy(&pdwstate), ::core::mem::transmute_copy(&pfpolicyencountered)).into()
        }
        unsafe extern "system" fn GetIESecurityState<Impl: IInternetZoneManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIESecurityState(::core::mem::transmute_copy(&frespectpolicy), ::core::mem::transmute_copy(&pdwstate), ::core::mem::transmute_copy(&pfpolicyencountered), ::core::mem::transmute_copy(&fnocache)).into()
        }
        unsafe extern "system" fn FixUnsecureSettings<Impl: IInternetZoneManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FixUnsecureSettings().into()
        }
        Self {
            base: IInternetZoneManagerExVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetZoneAttributesEx: GetZoneAttributesEx::<Impl, IMPL_OFFSET>,
            GetZoneSecurityState: GetZoneSecurityState::<Impl, IMPL_OFFSET>,
            GetIESecurityState: GetIESecurityState::<Impl, IMPL_OFFSET>,
            FixUnsecureSettings: FixUnsecureSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetZoneManagerEx2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMonikerPropImpl: Sized {
    fn PutProperty(&mut self, mkp: MONIKERPROPERTY, val: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMonikerPropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonikerPropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMonikerPropVtbl {
        unsafe extern "system" fn PutProperty<Impl: IMonikerPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mkp: MONIKERPROPERTY, val: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutProperty(::core::mem::transmute_copy(&mkp), ::core::mem::transmute_copy(&val)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), PutProperty: PutProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMonikerProp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistMonikerImpl: Sized {
    fn GetClassID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsDirty(&mut self) -> ::windows::core::Result<()>;
    fn Load(&mut self, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: ::core::option::Option<super::IMoniker>, pibc: ::core::option::Option<super::IBindCtx>, grfmode: u32) -> ::windows::core::Result<()>;
    fn Save(&mut self, pimkname: ::core::option::Option<super::IMoniker>, pbc: ::core::option::Option<super::IBindCtx>, fremember: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SaveCompleted(&mut self, pimkname: ::core::option::Option<super::IMoniker>, pibc: ::core::option::Option<super::IBindCtx>) -> ::windows::core::Result<()>;
    fn GetCurMoniker(&mut self) -> ::windows::core::Result<super::IMoniker>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPersistMonikerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistMonikerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistMonikerVtbl {
        unsafe extern "system" fn GetClassID<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclassid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirty<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsDirty().into()
        }
        unsafe extern "system" fn Load<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: ::windows::core::RawPtr, pibc: ::windows::core::RawPtr, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute_copy(&ffullyavailable), ::core::mem::transmute(&pimkname), ::core::mem::transmute(&pibc), ::core::mem::transmute_copy(&grfmode)).into()
        }
        unsafe extern "system" fn Save<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimkname: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, fremember: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(::core::mem::transmute(&pimkname), ::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&fremember)).into()
        }
        unsafe extern "system" fn SaveCompleted<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimkname: ::windows::core::RawPtr, pibc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveCompleted(::core::mem::transmute(&pimkname), ::core::mem::transmute(&pibc)).into()
        }
        unsafe extern "system" fn GetCurMoniker<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimkname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurMoniker() {
                ::core::result::Result::Ok(ok__) => {
                    *ppimkname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClassID: GetClassID::<Impl, IMPL_OFFSET>,
            IsDirty: IsDirty::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            Save: Save::<Impl, IMPL_OFFSET>,
            SaveCompleted: SaveCompleted::<Impl, IMPL_OFFSET>,
            GetCurMoniker: GetCurMoniker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistMoniker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
pub trait ISoftDistExtImpl: Sized {
    fn ProcessSoftDist(&mut self, szcdfurl: super::super::super::Foundation::PWSTR, psoftdistelement: ::core::option::Option<super::super::super::Data::Xml::MsXml::IXMLElement>, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::Result<()>;
    fn GetFirstCodeBase(&mut self, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()>;
    fn GetNextCodeBase(&mut self, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()>;
    fn AsyncInstallDistributionUnit(&mut self, pbc: ::core::option::Option<super::IBindCtx>, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
impl ISoftDistExtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftDistExtImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoftDistExtVtbl {
        unsafe extern "system" fn ProcessSoftDist<Impl: ISoftDistExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcdfurl: super::super::super::Foundation::PWSTR, psoftdistelement: ::windows::core::RawPtr, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessSoftDist(::core::mem::transmute_copy(&szcdfurl), ::core::mem::transmute(&psoftdistelement), ::core::mem::transmute_copy(&lpsdi)).into()
        }
        unsafe extern "system" fn GetFirstCodeBase<Impl: ISoftDistExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFirstCodeBase(::core::mem::transmute_copy(&szcodebase), ::core::mem::transmute_copy(&dwmaxsize)).into()
        }
        unsafe extern "system" fn GetNextCodeBase<Impl: ISoftDistExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextCodeBase(::core::mem::transmute_copy(&szcodebase), ::core::mem::transmute_copy(&dwmaxsize)).into()
        }
        unsafe extern "system" fn AsyncInstallDistributionUnit<Impl: ISoftDistExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncInstallDistributionUnit(::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&lpcbh)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProcessSoftDist: ProcessSoftDist::<Impl, IMPL_OFFSET>,
            GetFirstCodeBase: GetFirstCodeBase::<Impl, IMPL_OFFSET>,
            GetNextCodeBase: GetNextCodeBase::<Impl, IMPL_OFFSET>,
            AsyncInstallDistributionUnit: AsyncInstallDistributionUnit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISoftDistExt as ::windows::core::Interface>::IID
    }
}
pub trait IUriBuilderFactoryImpl: Sized {
    fn CreateIUriBuilder(&mut self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder>;
    fn CreateInitializedIUriBuilder(&mut self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder>;
}
impl IUriBuilderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriBuilderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriBuilderFactoryVtbl {
        unsafe extern "system" fn CreateIUriBuilder<Impl: IUriBuilderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateIUriBuilder(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiuribuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInitializedIUriBuilder<Impl: IUriBuilderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInitializedIUriBuilder(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiuribuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateIUriBuilder: CreateIUriBuilder::<Impl, IMPL_OFFSET>,
            CreateInitializedIUriBuilder: CreateInitializedIUriBuilder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriBuilderFactory as ::windows::core::Interface>::IID
    }
}
pub trait IUriContainerImpl: Sized {
    fn GetIUri(&mut self) -> ::windows::core::Result<super::IUri>;
}
impl IUriContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriContainerVtbl {
        unsafe extern "system" fn GetIUri<Impl: IUriContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIUri() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIUri: GetIUri::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWinInetCacheHintsImpl: Sized {
    fn SetCacheExtension(&mut self, pwzext: super::super::super::Foundation::PWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWinInetCacheHintsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetCacheHintsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetCacheHintsVtbl {
        unsafe extern "system" fn SetCacheExtension<Impl: IWinInetCacheHintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzext: super::super::super::Foundation::PWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCacheExtension(::core::mem::transmute_copy(&pwzext), ::core::mem::transmute_copy(&pszcachefile), ::core::mem::transmute_copy(&pcbcachefile), ::core::mem::transmute_copy(&pdwwinineterror), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetCacheExtension: SetCacheExtension::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetCacheHints as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWinInetCacheHints2Impl: Sized + IWinInetCacheHintsImpl {
    fn SetCacheExtension2(&mut self, pwzext: super::super::super::Foundation::PWSTR, pwzcachefile: super::super::super::Foundation::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWinInetCacheHints2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetCacheHints2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetCacheHints2Vtbl {
        unsafe extern "system" fn SetCacheExtension2<Impl: IWinInetCacheHints2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzext: super::super::super::Foundation::PWSTR, pwzcachefile: super::super::super::Foundation::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCacheExtension2(::core::mem::transmute_copy(&pwzext), ::core::mem::transmute_copy(&pwzcachefile), ::core::mem::transmute_copy(&pcchcachefile), ::core::mem::transmute_copy(&pdwwinineterror), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base: IWinInetCacheHintsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetCacheExtension2: SetCacheExtension2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetCacheHints2 as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetFileStreamImpl: Sized {
    fn SetHandleForUnlock(&mut self, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::Result<()>;
    fn SetDeleteFile(&mut self, dwreserved: usize) -> ::windows::core::Result<()>;
}
impl IWinInetFileStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetFileStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetFileStreamVtbl {
        unsafe extern "system" fn SetHandleForUnlock<Impl: IWinInetFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandleForUnlock(::core::mem::transmute_copy(&hwininetlockhandle), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetDeleteFile<Impl: IWinInetFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeleteFile(::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetHandleForUnlock: SetHandleForUnlock::<Impl, IMPL_OFFSET>,
            SetDeleteFile: SetDeleteFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetFileStream as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetHttpInfoImpl: Sized + IWinInetInfoImpl {
    fn QueryInfo(&mut self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
impl IWinInetHttpInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetHttpInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetHttpInfoVtbl {
        unsafe extern "system" fn QueryInfo<Impl: IWinInetHttpInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInfo(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbbuf), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base: IWinInetInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), QueryInfo: QueryInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetHttpInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetHttpTimeoutsImpl: Sized {
    fn GetRequestTimeouts(&mut self, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::Result<()>;
}
impl IWinInetHttpTimeoutsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetHttpTimeoutsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetHttpTimeoutsVtbl {
        unsafe extern "system" fn GetRequestTimeouts<Impl: IWinInetHttpTimeoutsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRequestTimeouts(::core::mem::transmute_copy(&pdwconnecttimeout), ::core::mem::transmute_copy(&pdwsendtimeout), ::core::mem::transmute_copy(&pdwreceivetimeout)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetRequestTimeouts: GetRequestTimeouts::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetHttpTimeouts as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetInfoImpl: Sized {
    fn QueryOption(&mut self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::Result<()>;
}
impl IWinInetInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetInfoVtbl {
        unsafe extern "system" fn QueryOption<Impl: IWinInetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryOption(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbbuf)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), QueryOption: QueryOption::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowForBindingUIImpl: Sized {
    fn GetWindow(&mut self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWindowForBindingUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowForBindingUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowForBindingUIVtbl {
        unsafe extern "system" fn GetWindow<Impl: IWindowForBindingUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidreason: *const ::windows::core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow(::core::mem::transmute_copy(&rguidreason)) {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetWindow: GetWindow::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowForBindingUI as ::windows::core::Interface>::IID
    }
}
pub trait IWrappedProtocolImpl: Sized {
    fn GetWrapperCode(&mut self, pncode: *mut i32, dwreserved: usize) -> ::windows::core::Result<()>;
}
impl IWrappedProtocolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWrappedProtocolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWrappedProtocolVtbl {
        unsafe extern "system" fn GetWrapperCode<Impl: IWrappedProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncode: *mut i32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWrapperCode(::core::mem::transmute_copy(&pncode), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetWrapperCode: GetWrapperCode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWrappedProtocol as ::windows::core::Interface>::IID
    }
}
pub trait IZoneIdentifierImpl: Sized {
    fn GetId(&mut self) -> ::windows::core::Result<u32>;
    fn SetId(&mut self, dwzone: u32) -> ::windows::core::Result<()>;
    fn Remove(&mut self) -> ::windows::core::Result<()>;
}
impl IZoneIdentifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoneIdentifierImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IZoneIdentifierVtbl {
        unsafe extern "system" fn GetId<Impl: IZoneIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwzone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IZoneIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&dwzone)).into()
        }
        unsafe extern "system" fn Remove<Impl: IZoneIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetId: GetId::<Impl, IMPL_OFFSET>,
            SetId: SetId::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoneIdentifier as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IZoneIdentifier2Impl: Sized + IZoneIdentifierImpl {
    fn GetLastWriterPackageFamilyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn SetLastWriterPackageFamilyName(&mut self, packagefamilyname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RemoveLastWriterPackageFamilyName(&mut self) -> ::windows::core::Result<()>;
    fn GetAppZoneId(&mut self) -> ::windows::core::Result<u32>;
    fn SetAppZoneId(&mut self, zone: u32) -> ::windows::core::Result<()>;
    fn RemoveAppZoneId(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IZoneIdentifier2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoneIdentifier2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IZoneIdentifier2Vtbl {
        unsafe extern "system" fn GetLastWriterPackageFamilyName<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastWriterPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *packagefamilyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastWriterPackageFamilyName<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastWriterPackageFamilyName(::core::mem::transmute_copy(&packagefamilyname)).into()
        }
        unsafe extern "system" fn RemoveLastWriterPackageFamilyName<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLastWriterPackageFamilyName().into()
        }
        unsafe extern "system" fn GetAppZoneId<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppZoneId() {
                ::core::result::Result::Ok(ok__) => {
                    *zone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppZoneId<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zone: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppZoneId(::core::mem::transmute_copy(&zone)).into()
        }
        unsafe extern "system" fn RemoveAppZoneId<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAppZoneId().into()
        }
        Self {
            base: IZoneIdentifierVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLastWriterPackageFamilyName: GetLastWriterPackageFamilyName::<Impl, IMPL_OFFSET>,
            SetLastWriterPackageFamilyName: SetLastWriterPackageFamilyName::<Impl, IMPL_OFFSET>,
            RemoveLastWriterPackageFamilyName: RemoveLastWriterPackageFamilyName::<Impl, IMPL_OFFSET>,
            GetAppZoneId: GetAppZoneId::<Impl, IMPL_OFFSET>,
            SetAppZoneId: SetAppZoneId::<Impl, IMPL_OFFSET>,
            RemoveAppZoneId: RemoveAppZoneId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IZoneIdentifier2 as ::windows::core::Interface>::IID
    }
}
