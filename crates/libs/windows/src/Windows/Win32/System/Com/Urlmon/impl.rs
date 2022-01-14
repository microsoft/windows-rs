#[cfg(feature = "Win32_Foundation")]
pub trait IBindCallbackRedirect_Impl: Sized {
    fn Redirect(&mut self, lpcurl: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<i16>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBindCallbackRedirect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindCallbackRedirect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindCallbackRedirect_Vtbl {
        unsafe extern "system" fn Redirect<Impl: IBindCallbackRedirect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcurl: super::super::super::Foundation::PWSTR, vbcancel: *mut i16) -> ::windows::core::HRESULT {
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
pub trait IBindHttpSecurity_Impl: Sized {
    fn GetIgnoreCertMask(&mut self) -> ::windows::core::Result<u32>;
}
impl IBindHttpSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindHttpSecurity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindHttpSecurity_Vtbl {
        unsafe extern "system" fn GetIgnoreCertMask<Impl: IBindHttpSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwignorecertmask: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IBindProtocol_Impl: Sized {
    fn CreateBinding(&mut self, szurl: super::super::super::Foundation::PWSTR, pbc: ::core::option::Option<super::IBindCtx>) -> ::windows::core::Result<super::IBinding>;
}
#[cfg(feature = "Win32_Foundation")]
impl IBindProtocol_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindProtocol_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindProtocol_Vtbl {
        unsafe extern "system" fn CreateBinding<Impl: IBindProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: super::super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, ppb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICatalogFileInfo_Impl: Sized {
    fn GetCatalogFile(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PSTR>;
    fn GetJavaTrust(&mut self, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICatalogFileInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogFileInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICatalogFileInfo_Vtbl {
        unsafe extern "system" fn GetCatalogFile<Impl: ICatalogFileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcatalogfile: *mut super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCatalogFile() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcatalogfile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJavaTrust<Impl: ICatalogFileInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ICodeInstall_Impl: Sized + IWindowForBindingUI_Impl {
    fn OnCodeInstallProblem(&mut self, ulstatuscode: u32, szdestination: super::super::super::Foundation::PWSTR, szsource: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICodeInstall_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICodeInstall_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICodeInstall_Vtbl {
        unsafe extern "system" fn OnCodeInstallProblem<Impl: ICodeInstall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szdestination: super::super::super::Foundation::PWSTR, szsource: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCodeInstallProblem(::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute_copy(&szdestination), ::core::mem::transmute_copy(&szsource), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: IWindowForBindingUI_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnCodeInstallProblem: OnCodeInstallProblem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICodeInstall as ::windows::core::Interface>::IID
    }
}
pub trait IDataFilter_Impl: Sized {
    fn DoEncode(&mut self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn DoDecode(&mut self, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn SetEncodingLevel(&mut self, dwenclevel: u32) -> ::windows::core::Result<()>;
}
impl IDataFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataFilter_Vtbl {
        unsafe extern "system" fn DoEncode<Impl: IDataFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoEncode(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&linbuffersize), ::core::mem::transmute_copy(&pbinbuffer), ::core::mem::transmute_copy(&loutbuffersize), ::core::mem::transmute_copy(&pboutbuffer), ::core::mem::transmute_copy(&linbytesavailable), ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn DoDecode<Impl: IDataFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoDecode(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&linbuffersize), ::core::mem::transmute_copy(&pbinbuffer), ::core::mem::transmute_copy(&loutbuffersize), ::core::mem::transmute_copy(&pboutbuffer), ::core::mem::transmute_copy(&linbytesavailable), ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetEncodingLevel<Impl: IDataFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenclevel: u32) -> ::windows::core::HRESULT {
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
pub trait IEncodingFilterFactory_Impl: Sized {
    fn FindBestFilter(&mut self, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, info: DATAINFO) -> ::windows::core::Result<IDataFilter>;
    fn GetDefaultFilter(&mut self, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<IDataFilter>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEncodingFilterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEncodingFilterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEncodingFilterFactory_Vtbl {
        unsafe extern "system" fn FindBestFilter<Impl: IEncodingFilterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, info: DATAINFO, ppdf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindBestFilter(::core::mem::transmute_copy(&pwzcodein), ::core::mem::transmute_copy(&pwzcodeout), ::core::mem::transmute_copy(&info)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdf = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFilter<Impl: IEncodingFilterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, ppdf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGetBindHandle_Impl: Sized {
    fn GetBindHandle(&mut self, enumrequestedhandle: BINDHANDLETYPES) -> ::windows::core::Result<super::super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGetBindHandle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetBindHandle_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetBindHandle_Vtbl {
        unsafe extern "system" fn GetBindHandle<Impl: IGetBindHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumrequestedhandle: BINDHANDLETYPES, prethandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
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
pub trait IHttpNegotiate_Impl: Sized {
    fn BeginningTransaction(&mut self, szurl: super::super::super::Foundation::PWSTR, szheaders: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn OnResponse(&mut self, dwresponsecode: u32, szresponseheaders: super::super::super::Foundation::PWSTR, szrequestheaders: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHttpNegotiate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNegotiate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpNegotiate_Vtbl {
        unsafe extern "system" fn BeginningTransaction<Impl: IHttpNegotiate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: super::super::super::Foundation::PWSTR, szheaders: super::super::super::Foundation::PWSTR, dwreserved: u32, pszadditionalheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginningTransaction(::core::mem::transmute_copy(&szurl), ::core::mem::transmute_copy(&szheaders), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pszadditionalheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResponse<Impl: IHttpNegotiate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwresponsecode: u32, szresponseheaders: super::super::super::Foundation::PWSTR, szrequestheaders: super::super::super::Foundation::PWSTR, pszadditionalrequestheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
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
pub trait IHttpNegotiate2_Impl: Sized + IHttpNegotiate_Impl {
    fn GetRootSecurityId(&mut self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHttpNegotiate2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNegotiate2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpNegotiate2_Vtbl {
        unsafe extern "system" fn GetRootSecurityId<Impl: IHttpNegotiate2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRootSecurityId(::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base: IHttpNegotiate_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetRootSecurityId: GetRootSecurityId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNegotiate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHttpNegotiate3_Impl: Sized + IHttpNegotiate_Impl + IHttpNegotiate2_Impl {
    fn GetSerializedClientCertContext(&mut self, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHttpNegotiate3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNegotiate3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpNegotiate3_Vtbl {
        unsafe extern "system" fn GetSerializedClientCertContext<Impl: IHttpNegotiate3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSerializedClientCertContext(::core::mem::transmute_copy(&ppbcert), ::core::mem::transmute_copy(&pcbcert)).into()
        }
        Self {
            base: IHttpNegotiate2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSerializedClientCertContext: GetSerializedClientCertContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpNegotiate3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHttpSecurity_Impl: Sized + IWindowForBindingUI_Impl {
    fn OnSecurityProblem(&mut self, dwproblem: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHttpSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpSecurity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHttpSecurity_Vtbl {
        unsafe extern "system" fn OnSecurityProblem<Impl: IHttpSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproblem: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSecurityProblem(::core::mem::transmute_copy(&dwproblem)).into()
        }
        Self { base: IWindowForBindingUI_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnSecurityProblem: OnSecurityProblem::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHttpSecurity as ::windows::core::Interface>::IID
    }
}
pub trait IInternet_Impl: Sized {}
impl IInternet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternet_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternet_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternet as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IInternetBindInfo_Impl: Sized {
    fn GetBindInfo(&mut self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::Result<()>;
    fn GetBindString(&mut self, ulstringtype: u32, ppwzstr: *mut super::super::super::Foundation::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IInternetBindInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetBindInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetBindInfo_Vtbl {
        unsafe extern "system" fn GetBindInfo<Impl: IInternetBindInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBindInfo(::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo)).into()
        }
        unsafe extern "system" fn GetBindString<Impl: IInternetBindInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstringtype: u32, ppwzstr: *mut super::super::super::Foundation::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IInternetBindInfoEx_Impl: Sized + IInternetBindInfo_Impl {
    fn GetBindInfoEx(&mut self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IInternetBindInfoEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetBindInfoEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetBindInfoEx_Vtbl {
        unsafe extern "system" fn GetBindInfoEx<Impl: IInternetBindInfoEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBindInfoEx(::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&grfbindf2), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base: IInternetBindInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetBindInfoEx: GetBindInfoEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetBindInfoEx as ::windows::core::Interface>::IID
    }
}
pub trait IInternetHostSecurityManager_Impl: Sized {
    fn GetSecurityId(&mut self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
    fn ProcessUrlAction(&mut self, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn QueryCustomPolicy(&mut self, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
impl IInternetHostSecurityManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetHostSecurityManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetHostSecurityManager_Vtbl {
        unsafe extern "system" fn GetSecurityId<Impl: IInternetHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSecurityId(::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn ProcessUrlAction<Impl: IInternetHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUrlAction(::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicy<Impl: IInternetHostSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT {
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
pub trait IInternetPriority_Impl: Sized {
    fn SetPriority(&mut self, npriority: i32) -> ::windows::core::Result<()>;
    fn GetPriority(&mut self) -> ::windows::core::Result<i32>;
}
impl IInternetPriority_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetPriority_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetPriority_Vtbl {
        unsafe extern "system" fn SetPriority<Impl: IInternetPriority_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(::core::mem::transmute_copy(&npriority)).into()
        }
        unsafe extern "system" fn GetPriority<Impl: IInternetPriority_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IInternetProtocol_Impl: Sized + IInternetProtocolRoot_Impl {
    fn Read(&mut self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::Result<()>;
    fn Seek(&mut self, dlibmove: i64, dworigin: u32) -> ::windows::core::Result<u64>;
    fn LockRequest(&mut self, dwoptions: u32) -> ::windows::core::Result<()>;
    fn UnlockRequest(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocol_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocol_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocol_Vtbl {
        unsafe extern "system" fn Read<Impl: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread)).into()
        }
        unsafe extern "system" fn Seek<Impl: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&dworigin)) {
                ::core::result::Result::Ok(ok__) => {
                    *plibnewposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRequest<Impl: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockRequest(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn UnlockRequest<Impl: IInternetProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnlockRequest().into()
        }
        Self {
            base: IInternetProtocolRoot_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IInternetProtocolEx_Impl: Sized + IInternetProtocolRoot_Impl + IInternetProtocol_Impl {
    fn StartEx(&mut self, puri: ::core::option::Option<super::IUri>, poiprotsink: ::core::option::Option<IInternetProtocolSink>, poibindinfo: ::core::option::Option<IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolEx_Vtbl {
        unsafe extern "system" fn StartEx<Impl: IInternetProtocolEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartEx(::core::mem::transmute(&puri), ::core::mem::transmute(&poiprotsink), ::core::mem::transmute(&poibindinfo), ::core::mem::transmute_copy(&grfpi), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base: IInternetProtocol_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), StartEx: StartEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetProtocolEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetProtocolInfo_Impl: Sized {
    fn ParseUrl(&mut self, pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn CombineUrl(&mut self, pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn CompareUrl(&mut self, pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwcompareflags: u32) -> ::windows::core::Result<()>;
    fn QueryInfo(&mut self, pwzurl: super::super::super::Foundation::PWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolInfo_Vtbl {
        unsafe extern "system" fn ParseUrl<Impl: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseUrl(::core::mem::transmute_copy(&pwzurl), ::core::mem::transmute_copy(&parseaction), ::core::mem::transmute_copy(&dwparseflags), ::core::mem::transmute_copy(&pwzresult), ::core::mem::transmute_copy(&cchresult), ::core::mem::transmute_copy(&pcchresult), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn CombineUrl<Impl: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CombineUrl(::core::mem::transmute_copy(&pwzbaseurl), ::core::mem::transmute_copy(&pwzrelativeurl), ::core::mem::transmute_copy(&dwcombineflags), ::core::mem::transmute_copy(&pwzresult), ::core::mem::transmute_copy(&cchresult), ::core::mem::transmute_copy(&pcchresult), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn CompareUrl<Impl: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwcompareflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompareUrl(::core::mem::transmute_copy(&pwzurl1), ::core::mem::transmute_copy(&pwzurl2), ::core::mem::transmute_copy(&dwcompareflags)).into()
        }
        unsafe extern "system" fn QueryInfo<Impl: IInternetProtocolInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl: super::super::super::Foundation::PWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
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
pub trait IInternetProtocolRoot_Impl: Sized {
    fn Start(&mut self, szurl: super::super::super::Foundation::PWSTR, poiprotsink: ::core::option::Option<IInternetProtocolSink>, poibindinfo: ::core::option::Option<IInternetBindInfo>, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::Result<()>;
    fn Continue(&mut self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()>;
    fn Abort(&mut self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()>;
    fn Terminate(&mut self, dwoptions: u32) -> ::windows::core::Result<()>;
    fn Suspend(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolRoot_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolRoot_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolRoot_Vtbl {
        unsafe extern "system" fn Start<Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: super::super::super::Foundation::PWSTR, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&szurl), ::core::mem::transmute(&poiprotsink), ::core::mem::transmute(&poibindinfo), ::core::mem::transmute_copy(&grfpi), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Continue<Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Continue(::core::mem::transmute_copy(&pprotocoldata)).into()
        }
        unsafe extern "system" fn Abort<Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort(::core::mem::transmute_copy(&hrreason), ::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn Terminate<Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate(::core::mem::transmute_copy(&dwoptions)).into()
        }
        unsafe extern "system" fn Suspend<Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Suspend().into()
        }
        unsafe extern "system" fn Resume<Impl: IInternetProtocolRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IInternetProtocolSink_Impl: Sized {
    fn Switch(&mut self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()>;
    fn ReportProgress(&mut self, ulstatuscode: u32, szstatustext: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ReportData(&mut self, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()>;
    fn ReportResult(&mut self, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetProtocolSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolSink_Vtbl {
        unsafe extern "system" fn Switch<Impl: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Switch(::core::mem::transmute_copy(&pprotocoldata)).into()
        }
        unsafe extern "system" fn ReportProgress<Impl: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szstatustext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportProgress(::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute_copy(&szstatustext)).into()
        }
        unsafe extern "system" fn ReportData<Impl: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportData(::core::mem::transmute_copy(&grfbscf), ::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax)).into()
        }
        unsafe extern "system" fn ReportResult<Impl: IInternetProtocolSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
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
pub trait IInternetProtocolSinkStackable_Impl: Sized {
    fn SwitchSink(&mut self, poiprotsink: ::core::option::Option<IInternetProtocolSink>) -> ::windows::core::Result<()>;
    fn CommitSwitch(&mut self) -> ::windows::core::Result<()>;
    fn RollbackSwitch(&mut self) -> ::windows::core::Result<()>;
}
impl IInternetProtocolSinkStackable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolSinkStackable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetProtocolSinkStackable_Vtbl {
        unsafe extern "system" fn SwitchSink<Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poiprotsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwitchSink(::core::mem::transmute(&poiprotsink)).into()
        }
        unsafe extern "system" fn CommitSwitch<Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitSwitch().into()
        }
        unsafe extern "system" fn RollbackSwitch<Impl: IInternetProtocolSinkStackable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IInternetSecurityManager_Impl: Sized {
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
impl IInternetSecurityManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSecurityManager_Vtbl {
        unsafe extern "system" fn SetSecuritySite<Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecuritySite(::core::mem::transmute(&psite)).into()
        }
        unsafe extern "system" fn GetSecuritySite<Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecuritySite() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsite = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapUrlToZone<Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MapUrlToZone(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pdwzone), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetSecurityId<Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSecurityId(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn ProcessUrlAction<Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUrlAction(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicy<Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryCustomPolicy(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetZoneMapping<Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, lpszpattern: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneMapping(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&lpszpattern), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneMappings<Impl: IInternetSecurityManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, ppenumstring: *mut ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
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
pub trait IInternetSecurityManagerEx_Impl: Sized + IInternetSecurityManager_Impl {
    fn ProcessUrlActionEx(&mut self, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSecurityManagerEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityManagerEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSecurityManagerEx_Vtbl {
        unsafe extern "system" fn ProcessUrlActionEx<Impl: IInternetSecurityManagerEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUrlActionEx(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        Self {
            base: IInternetSecurityManager_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ProcessUrlActionEx: ProcessUrlActionEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetSecurityManagerEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetSecurityManagerEx2_Impl: Sized + IInternetSecurityManager_Impl + IInternetSecurityManagerEx_Impl {
    fn MapUrlToZoneEx2(&mut self, puri: ::core::option::Option<super::IUri>, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut super::super::super::Foundation::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
    fn ProcessUrlActionEx2(&mut self, puri: ::core::option::Option<super::IUri>, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetSecurityIdEx2(&mut self, puri: ::core::option::Option<super::IUri>, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>;
    fn QueryCustomPolicyEx2(&mut self, puri: ::core::option::Option<super::IUri>, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSecurityManagerEx2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityManagerEx2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSecurityManagerEx2_Vtbl {
        unsafe extern "system" fn MapUrlToZoneEx2<Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut super::super::super::Foundation::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MapUrlToZoneEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&pdwzone), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppwszmappedurl), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn ProcessUrlActionEx2<Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessUrlActionEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&pdwoutflags)).into()
        }
        unsafe extern "system" fn GetSecurityIdEx2<Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSecurityIdEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&pbsecurityid), ::core::mem::transmute_copy(&pcbsecurityid), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn QueryCustomPolicyEx2<Impl: IInternetSecurityManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryCustomPolicyEx2(::core::mem::transmute(&puri), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&cbcontext), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base: IInternetSecurityManagerEx_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IInternetSecurityMgrSite_Impl: Sized {
    fn GetWindow(&mut self) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
    fn EnableModeless(&mut self, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSecurityMgrSite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityMgrSite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSecurityMgrSite_Vtbl {
        unsafe extern "system" fn GetWindow<Impl: IInternetSecurityMgrSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableModeless<Impl: IInternetSecurityMgrSite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
pub trait IInternetSession_Impl: Sized {
    fn RegisterNameSpace(&mut self, pcf: ::core::option::Option<super::IClassFactory>, rclsid: *const ::windows::core::GUID, pwzprotocol: super::super::super::Foundation::PWSTR, cpatterns: u32, ppwzpatterns: *const super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::Result<()>;
    fn UnregisterNameSpace(&mut self, pcf: ::core::option::Option<super::IClassFactory>, pszprotocol: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RegisterMimeFilter(&mut self, pcf: ::core::option::Option<super::IClassFactory>, rclsid: *const ::windows::core::GUID, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn UnregisterMimeFilter(&mut self, pcf: ::core::option::Option<super::IClassFactory>, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn CreateBinding(&mut self, pbc: ::core::option::Option<super::IBindCtx>, szurl: super::super::super::Foundation::PWSTR, punkouter: ::core::option::Option<::windows::core::IUnknown>, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>, ppoinetprot: *mut ::core::option::Option<IInternetProtocol>, dwoption: u32) -> ::windows::core::Result<()>;
    fn SetSessionOption(&mut self, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::Result<()>;
    fn GetSessionOption(&mut self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetSession_Vtbl {
        unsafe extern "system" fn RegisterNameSpace<Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, pwzprotocol: super::super::super::Foundation::PWSTR, cpatterns: u32, ppwzpatterns: *const super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterNameSpace(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&pwzprotocol), ::core::mem::transmute_copy(&cpatterns), ::core::mem::transmute_copy(&ppwzpatterns), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn UnregisterNameSpace<Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, pszprotocol: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterNameSpace(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&pszprotocol)).into()
        }
        unsafe extern "system" fn RegisterMimeFilter<Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterMimeFilter(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&pwztype)).into()
        }
        unsafe extern "system" fn UnregisterMimeFilter<Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterMimeFilter(::core::mem::transmute(&pcf), ::core::mem::transmute_copy(&pwztype)).into()
        }
        unsafe extern "system" fn CreateBinding<Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, punkouter: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void, ppoinetprot: *mut ::windows::core::RawPtr, dwoption: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateBinding(::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&szurl), ::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&ppunk), ::core::mem::transmute_copy(&ppoinetprot), ::core::mem::transmute_copy(&dwoption)).into()
        }
        unsafe extern "system" fn SetSessionOption<Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionOption(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbufferlength), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn GetSessionOption<Impl: IInternetSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
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
pub trait IInternetThreadSwitch_Impl: Sized {
    fn Prepare(&mut self) -> ::windows::core::Result<()>;
    fn Continue(&mut self) -> ::windows::core::Result<()>;
}
impl IInternetThreadSwitch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetThreadSwitch_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetThreadSwitch_Vtbl {
        unsafe extern "system" fn Prepare<Impl: IInternetThreadSwitch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Prepare().into()
        }
        unsafe extern "system" fn Continue<Impl: IInternetThreadSwitch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IInternetZoneManager_Impl: Sized {
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
impl IInternetZoneManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetZoneManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetZoneManager_Vtbl {
        unsafe extern "system" fn GetZoneAttributes<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneAttributes(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes)).into()
        }
        unsafe extern "system" fn SetZoneAttributes<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneAttributes(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes)).into()
        }
        unsafe extern "system" fn GetZoneCustomPolicy<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneCustomPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn SetZoneCustomPolicy<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneCustomPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&guidkey), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn GetZoneActionPolicy<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneActionPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn SetZoneActionPolicy<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneActionPolicy(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg)).into()
        }
        unsafe extern "system" fn PromptAction<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwpromptflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PromptAction(::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&dwpromptflags)).into()
        }
        unsafe extern "system" fn LogAction<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwlogflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogAction(::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&dwlogflags)).into()
        }
        unsafe extern "system" fn CreateZoneEnumerator<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateZoneEnumerator(::core::mem::transmute_copy(&pdwenum), ::core::mem::transmute_copy(&pdwcount), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneAt<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneAt(::core::mem::transmute_copy(&dwenum), ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwzone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyZoneEnumerator<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenum: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyZoneEnumerator(::core::mem::transmute_copy(&dwenum)).into()
        }
        unsafe extern "system" fn CopyTemplatePoliciesToZone<Impl: IInternetZoneManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::HRESULT {
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
pub trait IInternetZoneManagerEx_Impl: Sized + IInternetZoneManager_Impl {
    fn GetZoneActionPolicyEx(&mut self, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetZoneActionPolicyEx(&mut self, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetZoneManagerEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetZoneManagerEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetZoneManagerEx_Vtbl {
        unsafe extern "system" fn GetZoneActionPolicyEx<Impl: IInternetZoneManagerEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneActionPolicyEx(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetZoneActionPolicyEx<Impl: IInternetZoneManagerEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoneActionPolicyEx(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&ppolicy), ::core::mem::transmute_copy(&cbpolicy), ::core::mem::transmute_copy(&urlzonereg), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: IInternetZoneManager_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetZoneActionPolicyEx: GetZoneActionPolicyEx::<Impl, IMPL_OFFSET>,
            SetZoneActionPolicyEx: SetZoneActionPolicyEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternetZoneManagerEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInternetZoneManagerEx2_Impl: Sized + IInternetZoneManager_Impl + IInternetZoneManagerEx_Impl {
    fn GetZoneAttributesEx(&mut self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetZoneSecurityState(&mut self, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetIESecurityState(&mut self, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FixUnsecureSettings(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IInternetZoneManagerEx2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetZoneManagerEx2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternetZoneManagerEx2_Vtbl {
        unsafe extern "system" fn GetZoneAttributesEx<Impl: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneAttributesEx(::core::mem::transmute_copy(&dwzone), ::core::mem::transmute_copy(&pzoneattributes), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetZoneSecurityState<Impl: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetZoneSecurityState(::core::mem::transmute_copy(&dwzoneindex), ::core::mem::transmute_copy(&frespectpolicy), ::core::mem::transmute_copy(&pdwstate), ::core::mem::transmute_copy(&pfpolicyencountered)).into()
        }
        unsafe extern "system" fn GetIESecurityState<Impl: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIESecurityState(::core::mem::transmute_copy(&frespectpolicy), ::core::mem::transmute_copy(&pdwstate), ::core::mem::transmute_copy(&pfpolicyencountered), ::core::mem::transmute_copy(&fnocache)).into()
        }
        unsafe extern "system" fn FixUnsecureSettings<Impl: IInternetZoneManagerEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FixUnsecureSettings().into()
        }
        Self {
            base: IInternetZoneManagerEx_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMonikerProp_Impl: Sized {
    fn PutProperty(&mut self, mkp: MONIKERPROPERTY, val: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMonikerProp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonikerProp_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMonikerProp_Vtbl {
        unsafe extern "system" fn PutProperty<Impl: IMonikerProp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mkp: MONIKERPROPERTY, val: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
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
pub trait IPersistMoniker_Impl: Sized {
    fn GetClassID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsDirty(&mut self) -> ::windows::core::Result<()>;
    fn Load(&mut self, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: ::core::option::Option<super::IMoniker>, pibc: ::core::option::Option<super::IBindCtx>, grfmode: u32) -> ::windows::core::Result<()>;
    fn Save(&mut self, pimkname: ::core::option::Option<super::IMoniker>, pbc: ::core::option::Option<super::IBindCtx>, fremember: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SaveCompleted(&mut self, pimkname: ::core::option::Option<super::IMoniker>, pibc: ::core::option::Option<super::IBindCtx>) -> ::windows::core::Result<()>;
    fn GetCurMoniker(&mut self) -> ::windows::core::Result<super::IMoniker>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPersistMoniker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistMoniker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistMoniker_Vtbl {
        unsafe extern "system" fn GetClassID<Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclassid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirty<Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsDirty().into()
        }
        unsafe extern "system" fn Load<Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: ::windows::core::RawPtr, pibc: ::windows::core::RawPtr, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(::core::mem::transmute_copy(&ffullyavailable), ::core::mem::transmute(&pimkname), ::core::mem::transmute(&pibc), ::core::mem::transmute_copy(&grfmode)).into()
        }
        unsafe extern "system" fn Save<Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimkname: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, fremember: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Save(::core::mem::transmute(&pimkname), ::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&fremember)).into()
        }
        unsafe extern "system" fn SaveCompleted<Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimkname: ::windows::core::RawPtr, pibc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveCompleted(::core::mem::transmute(&pimkname), ::core::mem::transmute(&pibc)).into()
        }
        unsafe extern "system" fn GetCurMoniker<Impl: IPersistMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimkname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISoftDistExt_Impl: Sized {
    fn ProcessSoftDist(&mut self, szcdfurl: super::super::super::Foundation::PWSTR, psoftdistelement: ::core::option::Option<super::super::super::Data::Xml::MsXml::IXMLElement>, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::Result<()>;
    fn GetFirstCodeBase(&mut self, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()>;
    fn GetNextCodeBase(&mut self, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::Result<()>;
    fn AsyncInstallDistributionUnit(&mut self, pbc: ::core::option::Option<super::IBindCtx>, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
impl ISoftDistExt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftDistExt_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISoftDistExt_Vtbl {
        unsafe extern "system" fn ProcessSoftDist<Impl: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcdfurl: super::super::super::Foundation::PWSTR, psoftdistelement: ::windows::core::RawPtr, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessSoftDist(::core::mem::transmute_copy(&szcdfurl), ::core::mem::transmute(&psoftdistelement), ::core::mem::transmute_copy(&lpsdi)).into()
        }
        unsafe extern "system" fn GetFirstCodeBase<Impl: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFirstCodeBase(::core::mem::transmute_copy(&szcodebase), ::core::mem::transmute_copy(&dwmaxsize)).into()
        }
        unsafe extern "system" fn GetNextCodeBase<Impl: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextCodeBase(::core::mem::transmute_copy(&szcodebase), ::core::mem::transmute_copy(&dwmaxsize)).into()
        }
        unsafe extern "system" fn AsyncInstallDistributionUnit<Impl: ISoftDistExt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::HRESULT {
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
pub trait IUriBuilderFactory_Impl: Sized {
    fn CreateIUriBuilder(&mut self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder>;
    fn CreateInitializedIUriBuilder(&mut self, dwflags: u32, dwreserved: usize) -> ::windows::core::Result<super::IUriBuilder>;
}
impl IUriBuilderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriBuilderFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriBuilderFactory_Vtbl {
        unsafe extern "system" fn CreateIUriBuilder<Impl: IUriBuilderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateIUriBuilder(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiuribuilder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInitializedIUriBuilder<Impl: IUriBuilderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IUriContainer_Impl: Sized {
    fn GetIUri(&mut self) -> ::windows::core::Result<super::IUri>;
}
impl IUriContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriContainer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriContainer_Vtbl {
        unsafe extern "system" fn GetIUri<Impl: IUriContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IWinInetCacheHints_Impl: Sized {
    fn SetCacheExtension(&mut self, pwzext: super::super::super::Foundation::PWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWinInetCacheHints_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetCacheHints_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetCacheHints_Vtbl {
        unsafe extern "system" fn SetCacheExtension<Impl: IWinInetCacheHints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzext: super::super::super::Foundation::PWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IWinInetCacheHints2_Impl: Sized + IWinInetCacheHints_Impl {
    fn SetCacheExtension2(&mut self, pwzext: super::super::super::Foundation::PWSTR, pwzcachefile: super::super::super::Foundation::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWinInetCacheHints2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetCacheHints2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetCacheHints2_Vtbl {
        unsafe extern "system" fn SetCacheExtension2<Impl: IWinInetCacheHints2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzext: super::super::super::Foundation::PWSTR, pwzcachefile: super::super::super::Foundation::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCacheExtension2(::core::mem::transmute_copy(&pwzext), ::core::mem::transmute_copy(&pwzcachefile), ::core::mem::transmute_copy(&pcchcachefile), ::core::mem::transmute_copy(&pdwwinineterror), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base: IWinInetCacheHints_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetCacheExtension2: SetCacheExtension2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetCacheHints2 as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetFileStream_Impl: Sized {
    fn SetHandleForUnlock(&mut self, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::Result<()>;
    fn SetDeleteFile(&mut self, dwreserved: usize) -> ::windows::core::Result<()>;
}
impl IWinInetFileStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetFileStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetFileStream_Vtbl {
        unsafe extern "system" fn SetHandleForUnlock<Impl: IWinInetFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandleForUnlock(::core::mem::transmute_copy(&hwininetlockhandle), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetDeleteFile<Impl: IWinInetFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows::core::HRESULT {
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
pub trait IWinInetHttpInfo_Impl: Sized + IWinInetInfo_Impl {
    fn QueryInfo(&mut self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
impl IWinInetHttpInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetHttpInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetHttpInfo_Vtbl {
        unsafe extern "system" fn QueryInfo<Impl: IWinInetHttpInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryInfo(::core::mem::transmute_copy(&dwoption), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbbuf), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base: IWinInetInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), QueryInfo: QueryInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetHttpInfo as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetHttpTimeouts_Impl: Sized {
    fn GetRequestTimeouts(&mut self, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::Result<()>;
}
impl IWinInetHttpTimeouts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetHttpTimeouts_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetHttpTimeouts_Vtbl {
        unsafe extern "system" fn GetRequestTimeouts<Impl: IWinInetHttpTimeouts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRequestTimeouts(::core::mem::transmute_copy(&pdwconnecttimeout), ::core::mem::transmute_copy(&pdwsendtimeout), ::core::mem::transmute_copy(&pdwreceivetimeout)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetRequestTimeouts: GetRequestTimeouts::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWinInetHttpTimeouts as ::windows::core::Interface>::IID
    }
}
pub trait IWinInetInfo_Impl: Sized {
    fn QueryOption(&mut self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::Result<()>;
}
impl IWinInetInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWinInetInfo_Vtbl {
        unsafe extern "system" fn QueryOption<Impl: IWinInetInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IWindowForBindingUI_Impl: Sized {
    fn GetWindow(&mut self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWindowForBindingUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowForBindingUI_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowForBindingUI_Vtbl {
        unsafe extern "system" fn GetWindow<Impl: IWindowForBindingUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidreason: *const ::windows::core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
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
pub trait IWrappedProtocol_Impl: Sized {
    fn GetWrapperCode(&mut self, pncode: *mut i32, dwreserved: usize) -> ::windows::core::Result<()>;
}
impl IWrappedProtocol_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWrappedProtocol_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWrappedProtocol_Vtbl {
        unsafe extern "system" fn GetWrapperCode<Impl: IWrappedProtocol_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncode: *mut i32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetWrapperCode(::core::mem::transmute_copy(&pncode), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetWrapperCode: GetWrapperCode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWrappedProtocol as ::windows::core::Interface>::IID
    }
}
pub trait IZoneIdentifier_Impl: Sized {
    fn GetId(&mut self) -> ::windows::core::Result<u32>;
    fn SetId(&mut self, dwzone: u32) -> ::windows::core::Result<()>;
    fn Remove(&mut self) -> ::windows::core::Result<()>;
}
impl IZoneIdentifier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoneIdentifier_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IZoneIdentifier_Vtbl {
        unsafe extern "system" fn GetId<Impl: IZoneIdentifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwzone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IZoneIdentifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetId(::core::mem::transmute_copy(&dwzone)).into()
        }
        unsafe extern "system" fn Remove<Impl: IZoneIdentifier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IZoneIdentifier2_Impl: Sized + IZoneIdentifier_Impl {
    fn GetLastWriterPackageFamilyName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
    fn SetLastWriterPackageFamilyName(&mut self, packagefamilyname: super::super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RemoveLastWriterPackageFamilyName(&mut self) -> ::windows::core::Result<()>;
    fn GetAppZoneId(&mut self) -> ::windows::core::Result<u32>;
    fn SetAppZoneId(&mut self, zone: u32) -> ::windows::core::Result<()>;
    fn RemoveAppZoneId(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IZoneIdentifier2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoneIdentifier2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IZoneIdentifier2_Vtbl {
        unsafe extern "system" fn GetLastWriterPackageFamilyName<Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastWriterPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *packagefamilyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastWriterPackageFamilyName<Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastWriterPackageFamilyName(::core::mem::transmute_copy(&packagefamilyname)).into()
        }
        unsafe extern "system" fn RemoveLastWriterPackageFamilyName<Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLastWriterPackageFamilyName().into()
        }
        unsafe extern "system" fn GetAppZoneId<Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppZoneId() {
                ::core::result::Result::Ok(ok__) => {
                    *zone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppZoneId<Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zone: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppZoneId(::core::mem::transmute_copy(&zone)).into()
        }
        unsafe extern "system" fn RemoveAppZoneId<Impl: IZoneIdentifier2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAppZoneId().into()
        }
        Self {
            base: IZoneIdentifier_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
