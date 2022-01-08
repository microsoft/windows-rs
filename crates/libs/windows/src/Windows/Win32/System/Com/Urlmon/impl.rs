pub trait IBindCallbackRedirectImpl: Sized {
    fn Redirect();
}
impl ::windows::core::RuntimeName for IBindCallbackRedirect {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IBindCallbackRedirect";
}
impl IBindCallbackRedirectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindCallbackRedirectImpl, const OFFSET: isize>() -> IBindCallbackRedirectVtbl {
        unsafe extern "system" fn Redirect<Impl: IBindCallbackRedirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcurl: super::super::super::Foundation::PWSTR, vbcancel: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Redirect(&*(&lpcurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&vbcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBindCallbackRedirect>, ::windows::core::GetTrustLevel, Redirect::<Impl, OFFSET>)
    }
}
pub trait IBindHttpSecurityImpl: Sized {
    fn GetIgnoreCertMask();
}
impl ::windows::core::RuntimeName for IBindHttpSecurity {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IBindHttpSecurity";
}
impl IBindHttpSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindHttpSecurityImpl, const OFFSET: isize>() -> IBindHttpSecurityVtbl {
        unsafe extern "system" fn GetIgnoreCertMask<Impl: IBindHttpSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwignorecertmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIgnoreCertMask(::core::mem::transmute_copy(&pdwignorecertmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBindHttpSecurity>, ::windows::core::GetTrustLevel, GetIgnoreCertMask::<Impl, OFFSET>)
    }
}
pub trait IBindProtocolImpl: Sized {
    fn CreateBinding();
}
impl ::windows::core::RuntimeName for IBindProtocol {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IBindProtocol";
}
impl IBindProtocolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindProtocolImpl, const OFFSET: isize>() -> IBindProtocolVtbl {
        unsafe extern "system" fn CreateBinding<Impl: IBindProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: super::super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, ppb: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBinding(&*(&szurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbc as *const <super::IBindCtx as ::windows::core::Abi>::Abi as *const <super::IBindCtx as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBindProtocol>, ::windows::core::GetTrustLevel, CreateBinding::<Impl, OFFSET>)
    }
}
pub trait ICatalogFileInfoImpl: Sized {
    fn GetCatalogFile();
    fn GetJavaTrust();
}
impl ::windows::core::RuntimeName for ICatalogFileInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.ICatalogFileInfo";
}
impl ICatalogFileInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogFileInfoImpl, const OFFSET: isize>() -> ICatalogFileInfoVtbl {
        unsafe extern "system" fn GetCatalogFile<Impl: ICatalogFileInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcatalogfile: *mut super::super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCatalogFile(::core::mem::transmute_copy(&ppszcatalogfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJavaTrust<Impl: ICatalogFileInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppjavatrust: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJavaTrust(::core::mem::transmute_copy(&ppjavatrust)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICatalogFileInfo>, ::windows::core::GetTrustLevel, GetCatalogFile::<Impl, OFFSET>, GetJavaTrust::<Impl, OFFSET>)
    }
}
pub trait ICodeInstallImpl: Sized + IWindowForBindingUIImpl {
    fn OnCodeInstallProblem();
}
impl ::windows::core::RuntimeName for ICodeInstall {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.ICodeInstall";
}
impl ICodeInstallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICodeInstallImpl, const OFFSET: isize>() -> ICodeInstallVtbl {
        unsafe extern "system" fn OnCodeInstallProblem<Impl: ICodeInstallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szdestination: super::super::super::Foundation::PWSTR, szsource: super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCodeInstallProblem(ulstatuscode, &*(&szdestination as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&szsource as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICodeInstall>, ::windows::core::GetTrustLevel, OnCodeInstallProblem::<Impl, OFFSET>)
    }
}
pub trait IDataFilterImpl: Sized {
    fn DoEncode();
    fn DoDecode();
    fn SetEncodingLevel();
}
impl ::windows::core::RuntimeName for IDataFilter {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IDataFilter";
}
impl IDataFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataFilterImpl, const OFFSET: isize>() -> IDataFilterVtbl {
        unsafe extern "system" fn DoEncode<Impl: IDataFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoEncode(dwflags, linbuffersize, pbinbuffer, loutbuffersize, ::core::mem::transmute_copy(&pboutbuffer), linbytesavailable, ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoDecode<Impl: IDataFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, linbuffersize: i32, pbinbuffer: *const u8, loutbuffersize: i32, pboutbuffer: *mut u8, linbytesavailable: i32, plinbytesread: *mut i32, ploutbyteswritten: *mut i32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoDecode(dwflags, linbuffersize, pbinbuffer, loutbuffersize, ::core::mem::transmute_copy(&pboutbuffer), linbytesavailable, ::core::mem::transmute_copy(&plinbytesread), ::core::mem::transmute_copy(&ploutbyteswritten), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncodingLevel<Impl: IDataFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenclevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEncodingLevel(dwenclevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataFilter>, ::windows::core::GetTrustLevel, DoEncode::<Impl, OFFSET>, DoDecode::<Impl, OFFSET>, SetEncodingLevel::<Impl, OFFSET>)
    }
}
pub trait IEncodingFilterFactoryImpl: Sized {
    fn FindBestFilter();
    fn GetDefaultFilter();
}
impl ::windows::core::RuntimeName for IEncodingFilterFactory {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IEncodingFilterFactory";
}
impl IEncodingFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEncodingFilterFactoryImpl, const OFFSET: isize>() -> IEncodingFilterFactoryVtbl {
        unsafe extern "system" fn FindBestFilter<Impl: IEncodingFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, info: DATAINFO, ppdf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindBestFilter(
                &*(&pwzcodein as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwzcodeout as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&info as *const <DATAINFO as ::windows::core::Abi>::Abi as *const <DATAINFO as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppdf),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFilter<Impl: IEncodingFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzcodein: super::super::super::Foundation::PWSTR, pwzcodeout: super::super::super::Foundation::PWSTR, ppdf: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultFilter(&*(&pwzcodein as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwzcodeout as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEncodingFilterFactory>, ::windows::core::GetTrustLevel, FindBestFilter::<Impl, OFFSET>, GetDefaultFilter::<Impl, OFFSET>)
    }
}
pub trait IGetBindHandleImpl: Sized {
    fn GetBindHandle();
}
impl ::windows::core::RuntimeName for IGetBindHandle {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IGetBindHandle";
}
impl IGetBindHandleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetBindHandleImpl, const OFFSET: isize>() -> IGetBindHandleVtbl {
        unsafe extern "system" fn GetBindHandle<Impl: IGetBindHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumrequestedhandle: BINDHANDLETYPES, prethandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindHandle(enumrequestedhandle, ::core::mem::transmute_copy(&prethandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGetBindHandle>, ::windows::core::GetTrustLevel, GetBindHandle::<Impl, OFFSET>)
    }
}
pub trait IHttpNegotiateImpl: Sized {
    fn BeginningTransaction();
    fn OnResponse();
}
impl ::windows::core::RuntimeName for IHttpNegotiate {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IHttpNegotiate";
}
impl IHttpNegotiateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNegotiateImpl, const OFFSET: isize>() -> IHttpNegotiateVtbl {
        unsafe extern "system" fn BeginningTransaction<Impl: IHttpNegotiateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: super::super::super::Foundation::PWSTR, szheaders: super::super::super::Foundation::PWSTR, dwreserved: u32, pszadditionalheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginningTransaction(&*(&szurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&szheaders as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwreserved, ::core::mem::transmute_copy(&pszadditionalheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResponse<Impl: IHttpNegotiateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwresponsecode: u32, szresponseheaders: super::super::super::Foundation::PWSTR, szrequestheaders: super::super::super::Foundation::PWSTR, pszadditionalrequestheaders: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnResponse(dwresponsecode, &*(&szresponseheaders as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&szrequestheaders as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pszadditionalrequestheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpNegotiate>, ::windows::core::GetTrustLevel, BeginningTransaction::<Impl, OFFSET>, OnResponse::<Impl, OFFSET>)
    }
}
pub trait IHttpNegotiate2Impl: Sized + IHttpNegotiateImpl {
    fn GetRootSecurityId();
}
impl ::windows::core::RuntimeName for IHttpNegotiate2 {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IHttpNegotiate2";
}
impl IHttpNegotiate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNegotiate2Impl, const OFFSET: isize>() -> IHttpNegotiate2Vtbl {
        unsafe extern "system" fn GetRootSecurityId<Impl: IHttpNegotiate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRootSecurityId(::core::mem::transmute_copy(&pbsecurityid), pcbsecurityid, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpNegotiate2>, ::windows::core::GetTrustLevel, GetRootSecurityId::<Impl, OFFSET>)
    }
}
pub trait IHttpNegotiate3Impl: Sized + IHttpNegotiate2Impl + IHttpNegotiateImpl {
    fn GetSerializedClientCertContext();
}
impl ::windows::core::RuntimeName for IHttpNegotiate3 {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IHttpNegotiate3";
}
impl IHttpNegotiate3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpNegotiate3Impl, const OFFSET: isize>() -> IHttpNegotiate3Vtbl {
        unsafe extern "system" fn GetSerializedClientCertContext<Impl: IHttpNegotiate3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbcert: *mut *mut u8, pcbcert: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSerializedClientCertContext(::core::mem::transmute_copy(&ppbcert), ::core::mem::transmute_copy(&pcbcert)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpNegotiate3>, ::windows::core::GetTrustLevel, GetSerializedClientCertContext::<Impl, OFFSET>)
    }
}
pub trait IHttpSecurityImpl: Sized + IWindowForBindingUIImpl {
    fn OnSecurityProblem();
}
impl ::windows::core::RuntimeName for IHttpSecurity {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IHttpSecurity";
}
impl IHttpSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHttpSecurityImpl, const OFFSET: isize>() -> IHttpSecurityVtbl {
        unsafe extern "system" fn OnSecurityProblem<Impl: IHttpSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproblem: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSecurityProblem(dwproblem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHttpSecurity>, ::windows::core::GetTrustLevel, OnSecurityProblem::<Impl, OFFSET>)
    }
}
pub trait IInternetImpl: Sized {}
impl ::windows::core::RuntimeName for IInternet {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternet";
}
impl IInternetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetImpl, const OFFSET: isize>() -> IInternetVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternet>, ::windows::core::GetTrustLevel)
    }
}
pub trait IInternetBindInfoImpl: Sized {
    fn GetBindInfo();
    fn GetBindString();
}
impl ::windows::core::RuntimeName for IInternetBindInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetBindInfo";
}
impl IInternetBindInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetBindInfoImpl, const OFFSET: isize>() -> IInternetBindInfoVtbl {
        unsafe extern "system" fn GetBindInfo<Impl: IInternetBindInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindInfo(::core::mem::transmute_copy(&grfbindf), &*(&pbindinfo as *const <super::BINDINFO as ::windows::core::Abi>::Abi as *const <super::BINDINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindString<Impl: IInternetBindInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstringtype: u32, ppwzstr: *mut super::super::super::Foundation::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindString(ulstringtype, ::core::mem::transmute_copy(&ppwzstr), cel, pcelfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetBindInfo>, ::windows::core::GetTrustLevel, GetBindInfo::<Impl, OFFSET>, GetBindString::<Impl, OFFSET>)
    }
}
pub trait IInternetBindInfoExImpl: Sized + IInternetBindInfoImpl {
    fn GetBindInfoEx();
}
impl ::windows::core::RuntimeName for IInternetBindInfoEx {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetBindInfoEx";
}
impl IInternetBindInfoExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetBindInfoExImpl, const OFFSET: isize>() -> IInternetBindInfoExVtbl {
        unsafe extern "system" fn GetBindInfoEx<Impl: IInternetBindInfoExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindInfoEx(::core::mem::transmute_copy(&grfbindf), &*(&pbindinfo as *const <super::BINDINFO as ::windows::core::Abi>::Abi as *const <super::BINDINFO as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&grfbindf2), ::core::mem::transmute_copy(&pdwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetBindInfoEx>, ::windows::core::GetTrustLevel, GetBindInfoEx::<Impl, OFFSET>)
    }
}
pub trait IInternetHostSecurityManagerImpl: Sized {
    fn GetSecurityId();
    fn ProcessUrlAction();
    fn QueryCustomPolicy();
}
impl ::windows::core::RuntimeName for IInternetHostSecurityManager {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetHostSecurityManager";
}
impl IInternetHostSecurityManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetHostSecurityManagerImpl, const OFFSET: isize>() -> IInternetHostSecurityManagerVtbl {
        unsafe extern "system" fn GetSecurityId<Impl: IInternetHostSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityId(::core::mem::transmute_copy(&pbsecurityid), pcbsecurityid, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessUrlAction<Impl: IInternetHostSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessUrlAction(dwaction, ::core::mem::transmute_copy(&ppolicy), cbpolicy, pcontext, cbcontext, dwflags, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryCustomPolicy<Impl: IInternetHostSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidkey: &::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryCustomPolicy(&*(&guidkey as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), pcontext, cbcontext, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetHostSecurityManager>, ::windows::core::GetTrustLevel, GetSecurityId::<Impl, OFFSET>, ProcessUrlAction::<Impl, OFFSET>, QueryCustomPolicy::<Impl, OFFSET>)
    }
}
pub trait IInternetPriorityImpl: Sized {
    fn SetPriority();
    fn GetPriority();
}
impl ::windows::core::RuntimeName for IInternetPriority {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetPriority";
}
impl IInternetPriorityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetPriorityImpl, const OFFSET: isize>() -> IInternetPriorityVtbl {
        unsafe extern "system" fn SetPriority<Impl: IInternetPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPriority(npriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPriority<Impl: IInternetPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPriority(::core::mem::transmute_copy(&pnpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetPriority>, ::windows::core::GetTrustLevel, SetPriority::<Impl, OFFSET>, GetPriority::<Impl, OFFSET>)
    }
}
pub trait IInternetProtocolImpl: Sized + IInternetProtocolRootImpl {
    fn Read();
    fn Seek();
    fn LockRequest();
    fn UnlockRequest();
}
impl ::windows::core::RuntimeName for IInternetProtocol {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetProtocol";
}
impl IInternetProtocolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolImpl, const OFFSET: isize>() -> IInternetProtocolVtbl {
        unsafe extern "system" fn Read<Impl: IInternetProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(::core::mem::transmute_copy(&pv), cb, ::core::mem::transmute_copy(&pcbread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Impl: IInternetProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: u32, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seek(dlibmove, dworigin, ::core::mem::transmute_copy(&plibnewposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockRequest<Impl: IInternetProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockRequest(dwoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockRequest<Impl: IInternetProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetProtocol>, ::windows::core::GetTrustLevel, Read::<Impl, OFFSET>, Seek::<Impl, OFFSET>, LockRequest::<Impl, OFFSET>, UnlockRequest::<Impl, OFFSET>)
    }
}
pub trait IInternetProtocolExImpl: Sized + IInternetProtocolImpl + IInternetProtocolRootImpl {
    fn StartEx();
}
impl ::windows::core::RuntimeName for IInternetProtocolEx {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetProtocolEx";
}
impl IInternetProtocolExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolExImpl, const OFFSET: isize>() -> IInternetProtocolExVtbl {
        unsafe extern "system" fn StartEx<Impl: IInternetProtocolExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartEx(
                &*(&puri as *const <super::IUri as ::windows::core::Abi>::Abi as *const <super::IUri as ::windows::core::DefaultType>::DefaultType),
                &*(&poiprotsink as *const <IInternetProtocolSink as ::windows::core::Abi>::Abi as *const <IInternetProtocolSink as ::windows::core::DefaultType>::DefaultType),
                &*(&poibindinfo as *const <IInternetBindInfo as ::windows::core::Abi>::Abi as *const <IInternetBindInfo as ::windows::core::DefaultType>::DefaultType),
                grfpi,
                &*(&dwreserved as *const <super::super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetProtocolEx>, ::windows::core::GetTrustLevel, StartEx::<Impl, OFFSET>)
    }
}
pub trait IInternetProtocolInfoImpl: Sized {
    fn ParseUrl();
    fn CombineUrl();
    fn CompareUrl();
    fn QueryInfo();
}
impl ::windows::core::RuntimeName for IInternetProtocolInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetProtocolInfo";
}
impl IInternetProtocolInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolInfoImpl, const OFFSET: isize>() -> IInternetProtocolInfoVtbl {
        unsafe extern "system" fn ParseUrl<Impl: IInternetProtocolInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl: super::super::super::Foundation::PWSTR, parseaction: PARSEACTION, dwparseflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParseUrl(&*(&pwzurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), parseaction, dwparseflags, ::core::mem::transmute_copy(&pwzresult), cchresult, ::core::mem::transmute_copy(&pcchresult), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CombineUrl<Impl: IInternetProtocolInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzbaseurl: super::super::super::Foundation::PWSTR, pwzrelativeurl: super::super::super::Foundation::PWSTR, dwcombineflags: u32, pwzresult: super::super::super::Foundation::PWSTR, cchresult: u32, pcchresult: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CombineUrl(
                &*(&pwzbaseurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwzrelativeurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwcombineflags,
                &*(&pwzresult as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchresult,
                ::core::mem::transmute_copy(&pcchresult),
                dwreserved,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompareUrl<Impl: IInternetProtocolInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl1: super::super::super::Foundation::PWSTR, pwzurl2: super::super::super::Foundation::PWSTR, dwcompareflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompareUrl(&*(&pwzurl1 as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwzurl2 as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwcompareflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryInfo<Impl: IInternetProtocolInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzurl: super::super::super::Foundation::PWSTR, oueryoption: QUERYOPTION, dwqueryflags: u32, pbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, pcbbuf: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInfo(&*(&pwzurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), oueryoption, dwqueryflags, &*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbbuffer, pcbbuf, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetProtocolInfo>, ::windows::core::GetTrustLevel, ParseUrl::<Impl, OFFSET>, CombineUrl::<Impl, OFFSET>, CompareUrl::<Impl, OFFSET>, QueryInfo::<Impl, OFFSET>)
    }
}
pub trait IInternetProtocolRootImpl: Sized {
    fn Start();
    fn Continue();
    fn Abort();
    fn Terminate();
    fn Suspend();
    fn Resume();
}
impl ::windows::core::RuntimeName for IInternetProtocolRoot {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetProtocolRoot";
}
impl IInternetProtocolRootVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolRootImpl, const OFFSET: isize>() -> IInternetProtocolRootVtbl {
        unsafe extern "system" fn Start<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szurl: super::super::super::Foundation::PWSTR, poiprotsink: ::windows::core::RawPtr, poibindinfo: ::windows::core::RawPtr, grfpi: u32, dwreserved: super::super::super::Foundation::HANDLE_PTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start(
                &*(&szurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&poiprotsink as *const <IInternetProtocolSink as ::windows::core::Abi>::Abi as *const <IInternetProtocolSink as ::windows::core::DefaultType>::DefaultType),
                &*(&poibindinfo as *const <IInternetBindInfo as ::windows::core::Abi>::Abi as *const <IInternetBindInfo as ::windows::core::DefaultType>::DefaultType),
                grfpi,
                &*(&dwreserved as *const <super::super::super::Foundation::HANDLE_PTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE_PTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Continue<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Continue(&*(&pprotocoldata as *const <PROTOCOLDATA as ::windows::core::Abi>::Abi as *const <PROTOCOLDATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abort(hrreason, dwoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminate(dwoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspend<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suspend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: IInternetProtocolRootImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetProtocolRoot>, ::windows::core::GetTrustLevel, Start::<Impl, OFFSET>, Continue::<Impl, OFFSET>, Abort::<Impl, OFFSET>, Terminate::<Impl, OFFSET>, Suspend::<Impl, OFFSET>, Resume::<Impl, OFFSET>)
    }
}
pub trait IInternetProtocolSinkImpl: Sized {
    fn Switch();
    fn ReportProgress();
    fn ReportData();
    fn ReportResult();
}
impl ::windows::core::RuntimeName for IInternetProtocolSink {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetProtocolSink";
}
impl IInternetProtocolSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolSinkImpl, const OFFSET: isize>() -> IInternetProtocolSinkVtbl {
        unsafe extern "system" fn Switch<Impl: IInternetProtocolSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Switch(&*(&pprotocoldata as *const <PROTOCOLDATA as ::windows::core::Abi>::Abi as *const <PROTOCOLDATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportProgress<Impl: IInternetProtocolSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstatuscode: u32, szstatustext: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportProgress(ulstatuscode, &*(&szstatustext as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportData<Impl: IInternetProtocolSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbscf: u32, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportData(grfbscf, ulprogress, ulprogressmax) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportResult<Impl: IInternetProtocolSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrresult: ::windows::core::HRESULT, dwerror: u32, szresult: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportResult(hrresult, dwerror, &*(&szresult as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetProtocolSink>, ::windows::core::GetTrustLevel, Switch::<Impl, OFFSET>, ReportProgress::<Impl, OFFSET>, ReportData::<Impl, OFFSET>, ReportResult::<Impl, OFFSET>)
    }
}
pub trait IInternetProtocolSinkStackableImpl: Sized {
    fn SwitchSink();
    fn CommitSwitch();
    fn RollbackSwitch();
}
impl ::windows::core::RuntimeName for IInternetProtocolSinkStackable {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetProtocolSinkStackable";
}
impl IInternetProtocolSinkStackableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetProtocolSinkStackableImpl, const OFFSET: isize>() -> IInternetProtocolSinkStackableVtbl {
        unsafe extern "system" fn SwitchSink<Impl: IInternetProtocolSinkStackableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poiprotsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SwitchSink(&*(&poiprotsink as *const <IInternetProtocolSink as ::windows::core::Abi>::Abi as *const <IInternetProtocolSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitSwitch<Impl: IInternetProtocolSinkStackableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitSwitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RollbackSwitch<Impl: IInternetProtocolSinkStackableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RollbackSwitch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetProtocolSinkStackable>, ::windows::core::GetTrustLevel, SwitchSink::<Impl, OFFSET>, CommitSwitch::<Impl, OFFSET>, RollbackSwitch::<Impl, OFFSET>)
    }
}
pub trait IInternetSecurityManagerImpl: Sized {
    fn SetSecuritySite();
    fn GetSecuritySite();
    fn MapUrlToZone();
    fn GetSecurityId();
    fn ProcessUrlAction();
    fn QueryCustomPolicy();
    fn SetZoneMapping();
    fn GetZoneMappings();
}
impl ::windows::core::RuntimeName for IInternetSecurityManager {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetSecurityManager";
}
impl IInternetSecurityManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityManagerImpl, const OFFSET: isize>() -> IInternetSecurityManagerVtbl {
        unsafe extern "system" fn SetSecuritySite<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psite: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSecuritySite(&*(&psite as *const <IInternetSecurityMgrSite as ::windows::core::Abi>::Abi as *const <IInternetSecurityMgrSite as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecuritySite<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecuritySite(::core::mem::transmute_copy(&ppsite)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapUrlToZone<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapUrlToZone(&*(&pwszurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwzone), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityId<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityId(&*(&pwszurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbsecurityid), pcbsecurityid, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessUrlAction<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessUrlAction(&*(&pwszurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwaction, ::core::mem::transmute_copy(&ppolicy), cbpolicy, pcontext, cbcontext, dwflags, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryCustomPolicy<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, guidkey: &::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryCustomPolicy(&*(&pwszurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&guidkey as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), pcontext, cbcontext, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoneMapping<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, lpszpattern: super::super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetZoneMapping(dwzone, &*(&lpszpattern as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetZoneMappings<Impl: IInternetSecurityManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, ppenumstring: *mut ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneMappings(dwzone, ::core::mem::transmute_copy(&ppenumstring), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInternetSecurityManager>,
            ::windows::core::GetTrustLevel,
            SetSecuritySite::<Impl, OFFSET>,
            GetSecuritySite::<Impl, OFFSET>,
            MapUrlToZone::<Impl, OFFSET>,
            GetSecurityId::<Impl, OFFSET>,
            ProcessUrlAction::<Impl, OFFSET>,
            QueryCustomPolicy::<Impl, OFFSET>,
            SetZoneMapping::<Impl, OFFSET>,
            GetZoneMappings::<Impl, OFFSET>,
        )
    }
}
pub trait IInternetSecurityManagerExImpl: Sized + IInternetSecurityManagerImpl {
    fn ProcessUrlActionEx();
}
impl ::windows::core::RuntimeName for IInternetSecurityManagerEx {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetSecurityManagerEx";
}
impl IInternetSecurityManagerExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityManagerExImpl, const OFFSET: isize>() -> IInternetSecurityManagerExVtbl {
        unsafe extern "system" fn ProcessUrlActionEx<Impl: IInternetSecurityManagerExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessUrlActionEx(&*(&pwszurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwaction, ::core::mem::transmute_copy(&ppolicy), cbpolicy, pcontext, cbcontext, dwflags, dwreserved, ::core::mem::transmute_copy(&pdwoutflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetSecurityManagerEx>, ::windows::core::GetTrustLevel, ProcessUrlActionEx::<Impl, OFFSET>)
    }
}
pub trait IInternetSecurityManagerEx2Impl: Sized + IInternetSecurityManagerExImpl + IInternetSecurityManagerImpl {
    fn MapUrlToZoneEx2();
    fn ProcessUrlActionEx2();
    fn GetSecurityIdEx2();
    fn QueryCustomPolicyEx2();
}
impl ::windows::core::RuntimeName for IInternetSecurityManagerEx2 {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetSecurityManagerEx2";
}
impl IInternetSecurityManagerEx2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityManagerEx2Impl, const OFFSET: isize>() -> IInternetSecurityManagerEx2Vtbl {
        unsafe extern "system" fn MapUrlToZoneEx2<Impl: IInternetSecurityManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pdwzone: *mut u32, dwflags: u32, ppwszmappedurl: *mut super::super::super::Foundation::PWSTR, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapUrlToZoneEx2(&*(&puri as *const <super::IUri as ::windows::core::Abi>::Abi as *const <super::IUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwzone), dwflags, ::core::mem::transmute_copy(&ppwszmappedurl), ::core::mem::transmute_copy(&pdwoutflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessUrlActionEx2<Impl: IInternetSecurityManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: usize, pdwoutflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessUrlActionEx2(&*(&puri as *const <super::IUri as ::windows::core::Abi>::Abi as *const <super::IUri as ::windows::core::DefaultType>::DefaultType), dwaction, ::core::mem::transmute_copy(&ppolicy), cbpolicy, pcontext, cbcontext, dwflags, dwreserved, ::core::mem::transmute_copy(&pdwoutflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSecurityIdEx2<Impl: IInternetSecurityManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityIdEx2(&*(&puri as *const <super::IUri as ::windows::core::Abi>::Abi as *const <super::IUri as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbsecurityid), pcbsecurityid, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryCustomPolicyEx2<Impl: IInternetSecurityManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, guidkey: &::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryCustomPolicyEx2(&*(&puri as *const <super::IUri as ::windows::core::Abi>::Abi as *const <super::IUri as ::windows::core::DefaultType>::DefaultType), &*(&guidkey as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), pcontext, cbcontext, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetSecurityManagerEx2>, ::windows::core::GetTrustLevel, MapUrlToZoneEx2::<Impl, OFFSET>, ProcessUrlActionEx2::<Impl, OFFSET>, GetSecurityIdEx2::<Impl, OFFSET>, QueryCustomPolicyEx2::<Impl, OFFSET>)
    }
}
pub trait IInternetSecurityMgrSiteImpl: Sized {
    fn GetWindow();
    fn EnableModeless();
}
impl ::windows::core::RuntimeName for IInternetSecurityMgrSite {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetSecurityMgrSite";
}
impl IInternetSecurityMgrSiteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSecurityMgrSiteImpl, const OFFSET: isize>() -> IInternetSecurityMgrSiteVtbl {
        unsafe extern "system" fn GetWindow<Impl: IInternetSecurityMgrSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableModeless<Impl: IInternetSecurityMgrSiteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableModeless(&*(&fenable as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetSecurityMgrSite>, ::windows::core::GetTrustLevel, GetWindow::<Impl, OFFSET>, EnableModeless::<Impl, OFFSET>)
    }
}
pub trait IInternetSessionImpl: Sized {
    fn RegisterNameSpace();
    fn UnregisterNameSpace();
    fn RegisterMimeFilter();
    fn UnregisterMimeFilter();
    fn CreateBinding();
    fn SetSessionOption();
    fn GetSessionOption();
}
impl ::windows::core::RuntimeName for IInternetSession {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetSession";
}
impl IInternetSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetSessionImpl, const OFFSET: isize>() -> IInternetSessionVtbl {
        unsafe extern "system" fn RegisterNameSpace<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, rclsid: &::windows::core::GUID, pwzprotocol: super::super::super::Foundation::PWSTR, cpatterns: u32, ppwzpatterns: *const super::super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterNameSpace(
                &*(&pcf as *const <super::IClassFactory as ::windows::core::Abi>::Abi as *const <super::IClassFactory as ::windows::core::DefaultType>::DefaultType),
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pwzprotocol as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cpatterns,
                &*(&ppwzpatterns as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwreserved,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNameSpace<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, pszprotocol: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterNameSpace(&*(&pcf as *const <super::IClassFactory as ::windows::core::Abi>::Abi as *const <super::IClassFactory as ::windows::core::DefaultType>::DefaultType), &*(&pszprotocol as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterMimeFilter<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, rclsid: &::windows::core::GUID, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterMimeFilter(
                &*(&pcf as *const <super::IClassFactory as ::windows::core::Abi>::Abi as *const <super::IClassFactory as ::windows::core::DefaultType>::DefaultType),
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pwztype as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterMimeFilter<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcf: ::windows::core::RawPtr, pwztype: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterMimeFilter(&*(&pcf as *const <super::IClassFactory as ::windows::core::Abi>::Abi as *const <super::IClassFactory as ::windows::core::DefaultType>::DefaultType), &*(&pwztype as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBinding<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, szurl: super::super::super::Foundation::PWSTR, punkouter: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void, ppoinetprot: *mut ::windows::core::RawPtr, dwoption: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBinding(
                &*(&pbc as *const <super::IBindCtx as ::windows::core::Abi>::Abi as *const <super::IBindCtx as ::windows::core::DefaultType>::DefaultType),
                &*(&szurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppunk),
                ::core::mem::transmute_copy(&ppoinetprot),
                dwoption,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionOption<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *const ::core::ffi::c_void, dwbufferlength: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSessionOption(dwoption, &*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwbufferlength, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionOption<Impl: IInternetSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pdwbufferlength: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionOption(dwoption, &*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), pdwbufferlength, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInternetSession>,
            ::windows::core::GetTrustLevel,
            RegisterNameSpace::<Impl, OFFSET>,
            UnregisterNameSpace::<Impl, OFFSET>,
            RegisterMimeFilter::<Impl, OFFSET>,
            UnregisterMimeFilter::<Impl, OFFSET>,
            CreateBinding::<Impl, OFFSET>,
            SetSessionOption::<Impl, OFFSET>,
            GetSessionOption::<Impl, OFFSET>,
        )
    }
}
pub trait IInternetThreadSwitchImpl: Sized {
    fn Prepare();
    fn Continue();
}
impl ::windows::core::RuntimeName for IInternetThreadSwitch {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetThreadSwitch";
}
impl IInternetThreadSwitchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetThreadSwitchImpl, const OFFSET: isize>() -> IInternetThreadSwitchVtbl {
        unsafe extern "system" fn Prepare<Impl: IInternetThreadSwitchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Prepare() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Continue<Impl: IInternetThreadSwitchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Continue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetThreadSwitch>, ::windows::core::GetTrustLevel, Prepare::<Impl, OFFSET>, Continue::<Impl, OFFSET>)
    }
}
pub trait IInternetZoneManagerImpl: Sized {
    fn GetZoneAttributes();
    fn SetZoneAttributes();
    fn GetZoneCustomPolicy();
    fn SetZoneCustomPolicy();
    fn GetZoneActionPolicy();
    fn SetZoneActionPolicy();
    fn PromptAction();
    fn LogAction();
    fn CreateZoneEnumerator();
    fn GetZoneAt();
    fn DestroyZoneEnumerator();
    fn CopyTemplatePoliciesToZone();
}
impl ::windows::core::RuntimeName for IInternetZoneManager {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetZoneManager";
}
impl IInternetZoneManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetZoneManagerImpl, const OFFSET: isize>() -> IInternetZoneManagerVtbl {
        unsafe extern "system" fn GetZoneAttributes<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneAttributes(dwzone, &*(&pzoneattributes as *const <ZONEATTRIBUTES as ::windows::core::Abi>::Abi as *const <ZONEATTRIBUTES as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoneAttributes<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetZoneAttributes(dwzone, &*(&pzoneattributes as *const <ZONEATTRIBUTES as ::windows::core::Abi>::Abi as *const <ZONEATTRIBUTES as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetZoneCustomPolicy<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: &::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneCustomPolicy(dwzone, &*(&guidkey as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pppolicy), ::core::mem::transmute_copy(&pcbpolicy), urlzonereg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoneCustomPolicy<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, guidkey: &::windows::core::GUID, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetZoneCustomPolicy(dwzone, &*(&guidkey as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ppolicy, cbpolicy, urlzonereg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetZoneActionPolicy<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneActionPolicy(dwzone, dwaction, ::core::mem::transmute_copy(&ppolicy), cbpolicy, urlzonereg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoneActionPolicy<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetZoneActionPolicy(dwzone, dwaction, ppolicy, cbpolicy, urlzonereg) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromptAction<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, hwndparent: super::super::super::Foundation::HWND, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwpromptflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PromptAction(
                dwaction,
                &*(&hwndparent as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwsztext as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwpromptflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogAction<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaction: u32, pwszurl: super::super::super::Foundation::PWSTR, pwsztext: super::super::super::Foundation::PWSTR, dwlogflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogAction(dwaction, &*(&pwszurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwsztext as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwlogflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateZoneEnumerator<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateZoneEnumerator(::core::mem::transmute_copy(&pdwenum), ::core::mem::transmute_copy(&pdwcount), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetZoneAt<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenum: u32, dwindex: u32, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneAt(dwenum, dwindex, ::core::mem::transmute_copy(&pdwzone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyZoneEnumerator<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwenum: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyZoneEnumerator(dwenum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyTemplatePoliciesToZone<Impl: IInternetZoneManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyTemplatePoliciesToZone(dwtemplate, dwzone, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IInternetZoneManager>,
            ::windows::core::GetTrustLevel,
            GetZoneAttributes::<Impl, OFFSET>,
            SetZoneAttributes::<Impl, OFFSET>,
            GetZoneCustomPolicy::<Impl, OFFSET>,
            SetZoneCustomPolicy::<Impl, OFFSET>,
            GetZoneActionPolicy::<Impl, OFFSET>,
            SetZoneActionPolicy::<Impl, OFFSET>,
            PromptAction::<Impl, OFFSET>,
            LogAction::<Impl, OFFSET>,
            CreateZoneEnumerator::<Impl, OFFSET>,
            GetZoneAt::<Impl, OFFSET>,
            DestroyZoneEnumerator::<Impl, OFFSET>,
            CopyTemplatePoliciesToZone::<Impl, OFFSET>,
        )
    }
}
pub trait IInternetZoneManagerExImpl: Sized + IInternetZoneManagerImpl {
    fn GetZoneActionPolicyEx();
    fn SetZoneActionPolicyEx();
}
impl ::windows::core::RuntimeName for IInternetZoneManagerEx {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetZoneManagerEx";
}
impl IInternetZoneManagerExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetZoneManagerExImpl, const OFFSET: isize>() -> IInternetZoneManagerExVtbl {
        unsafe extern "system" fn GetZoneActionPolicyEx<Impl: IInternetZoneManagerExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *mut u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneActionPolicyEx(dwzone, dwaction, ::core::mem::transmute_copy(&ppolicy), cbpolicy, urlzonereg, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoneActionPolicyEx<Impl: IInternetZoneManagerExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, dwaction: u32, ppolicy: *const u8, cbpolicy: u32, urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetZoneActionPolicyEx(dwzone, dwaction, ppolicy, cbpolicy, urlzonereg, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetZoneManagerEx>, ::windows::core::GetTrustLevel, GetZoneActionPolicyEx::<Impl, OFFSET>, SetZoneActionPolicyEx::<Impl, OFFSET>)
    }
}
pub trait IInternetZoneManagerEx2Impl: Sized + IInternetZoneManagerExImpl + IInternetZoneManagerImpl {
    fn GetZoneAttributesEx();
    fn GetZoneSecurityState();
    fn GetIESecurityState();
    fn FixUnsecureSettings();
}
impl ::windows::core::RuntimeName for IInternetZoneManagerEx2 {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IInternetZoneManagerEx2";
}
impl IInternetZoneManagerEx2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternetZoneManagerEx2Impl, const OFFSET: isize>() -> IInternetZoneManagerEx2Vtbl {
        unsafe extern "system" fn GetZoneAttributesEx<Impl: IInternetZoneManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneAttributesEx(dwzone, &*(&pzoneattributes as *const <ZONEATTRIBUTES as ::windows::core::Abi>::Abi as *const <ZONEATTRIBUTES as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetZoneSecurityState<Impl: IInternetZoneManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzoneindex: u32, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetZoneSecurityState(dwzoneindex, &*(&frespectpolicy as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), pdwstate, &*(&pfpolicyencountered as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIESecurityState<Impl: IInternetZoneManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frespectpolicy: super::super::super::Foundation::BOOL, pdwstate: *mut u32, pfpolicyencountered: *mut super::super::super::Foundation::BOOL, fnocache: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIESecurityState(
                &*(&frespectpolicy as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                pdwstate,
                &*(&pfpolicyencountered as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&fnocache as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FixUnsecureSettings<Impl: IInternetZoneManagerEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FixUnsecureSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInternetZoneManagerEx2>, ::windows::core::GetTrustLevel, GetZoneAttributesEx::<Impl, OFFSET>, GetZoneSecurityState::<Impl, OFFSET>, GetIESecurityState::<Impl, OFFSET>, FixUnsecureSettings::<Impl, OFFSET>)
    }
}
pub trait IMonikerPropImpl: Sized {
    fn PutProperty();
}
impl ::windows::core::RuntimeName for IMonikerProp {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IMonikerProp";
}
impl IMonikerPropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonikerPropImpl, const OFFSET: isize>() -> IMonikerPropVtbl {
        unsafe extern "system" fn PutProperty<Impl: IMonikerPropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mkp: MONIKERPROPERTY, val: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PutProperty(mkp, &*(&val as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMonikerProp>, ::windows::core::GetTrustLevel, PutProperty::<Impl, OFFSET>)
    }
}
pub trait IPersistMonikerImpl: Sized {
    fn GetClassID();
    fn IsDirty();
    fn Load();
    fn Save();
    fn SaveCompleted();
    fn GetCurMoniker();
}
impl ::windows::core::RuntimeName for IPersistMoniker {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IPersistMoniker";
}
impl IPersistMonikerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistMonikerImpl, const OFFSET: isize>() -> IPersistMonikerVtbl {
        unsafe extern "system" fn GetClassID<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassID(::core::mem::transmute_copy(&pclassid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirty<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Load<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffullyavailable: super::super::super::Foundation::BOOL, pimkname: ::windows::core::RawPtr, pibc: ::windows::core::RawPtr, grfmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(
                &*(&ffullyavailable as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pimkname as *const <super::IMoniker as ::windows::core::Abi>::Abi as *const <super::IMoniker as ::windows::core::DefaultType>::DefaultType),
                &*(&pibc as *const <super::IBindCtx as ::windows::core::Abi>::Abi as *const <super::IBindCtx as ::windows::core::DefaultType>::DefaultType),
                grfmode,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Save<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimkname: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, fremember: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Save(&*(&pimkname as *const <super::IMoniker as ::windows::core::Abi>::Abi as *const <super::IMoniker as ::windows::core::DefaultType>::DefaultType), &*(&pbc as *const <super::IBindCtx as ::windows::core::Abi>::Abi as *const <super::IBindCtx as ::windows::core::DefaultType>::DefaultType), &*(&fremember as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveCompleted<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimkname: ::windows::core::RawPtr, pibc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveCompleted(&*(&pimkname as *const <super::IMoniker as ::windows::core::Abi>::Abi as *const <super::IMoniker as ::windows::core::DefaultType>::DefaultType), &*(&pibc as *const <super::IBindCtx as ::windows::core::Abi>::Abi as *const <super::IBindCtx as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurMoniker<Impl: IPersistMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppimkname: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurMoniker(::core::mem::transmute_copy(&ppimkname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersistMoniker>, ::windows::core::GetTrustLevel, GetClassID::<Impl, OFFSET>, IsDirty::<Impl, OFFSET>, Load::<Impl, OFFSET>, Save::<Impl, OFFSET>, SaveCompleted::<Impl, OFFSET>, GetCurMoniker::<Impl, OFFSET>)
    }
}
pub trait ISoftDistExtImpl: Sized {
    fn ProcessSoftDist();
    fn GetFirstCodeBase();
    fn GetNextCodeBase();
    fn AsyncInstallDistributionUnit();
}
impl ::windows::core::RuntimeName for ISoftDistExt {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.ISoftDistExt";
}
impl ISoftDistExtVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISoftDistExtImpl, const OFFSET: isize>() -> ISoftDistExtVtbl {
        unsafe extern "system" fn ProcessSoftDist<Impl: ISoftDistExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcdfurl: super::super::super::Foundation::PWSTR, psoftdistelement: ::windows::core::RawPtr, lpsdi: *mut SOFTDISTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessSoftDist(
                &*(&szcdfurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psoftdistelement as *const <super::super::super::Data::Xml::MsXml::IXMLElement as ::windows::core::Abi>::Abi as *const <super::super::super::Data::Xml::MsXml::IXMLElement as ::windows::core::DefaultType>::DefaultType),
                &*(&lpsdi as *const <SOFTDISTINFO as ::windows::core::Abi>::Abi as *const <SOFTDISTINFO as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstCodeBase<Impl: ISoftDistExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstCodeBase(&*(&szcodebase as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmaxsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextCodeBase<Impl: ISoftDistExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szcodebase: *const super::super::super::Foundation::PWSTR, dwmaxsize: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextCodeBase(&*(&szcodebase as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwmaxsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncInstallDistributionUnit<Impl: ISoftDistExtImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pvreserved: *const ::core::ffi::c_void, flags: u32, lpcbh: *const CODEBASEHOLD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncInstallDistributionUnit(&*(&pbc as *const <super::IBindCtx as ::windows::core::Abi>::Abi as *const <super::IBindCtx as ::windows::core::DefaultType>::DefaultType), &*(&pvreserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), flags, &*(&lpcbh as *const <CODEBASEHOLD as ::windows::core::Abi>::Abi as *const <CODEBASEHOLD as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISoftDistExt>, ::windows::core::GetTrustLevel, ProcessSoftDist::<Impl, OFFSET>, GetFirstCodeBase::<Impl, OFFSET>, GetNextCodeBase::<Impl, OFFSET>, AsyncInstallDistributionUnit::<Impl, OFFSET>)
    }
}
pub trait IUriBuilderFactoryImpl: Sized {
    fn CreateIUriBuilder();
    fn CreateInitializedIUriBuilder();
}
impl ::windows::core::RuntimeName for IUriBuilderFactory {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IUriBuilderFactory";
}
impl IUriBuilderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriBuilderFactoryImpl, const OFFSET: isize>() -> IUriBuilderFactoryVtbl {
        unsafe extern "system" fn CreateIUriBuilder<Impl: IUriBuilderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateIUriBuilder(dwflags, dwreserved, ::core::mem::transmute_copy(&ppiuribuilder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInitializedIUriBuilder<Impl: IUriBuilderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwreserved: usize, ppiuribuilder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInitializedIUriBuilder(dwflags, dwreserved, ::core::mem::transmute_copy(&ppiuribuilder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUriBuilderFactory>, ::windows::core::GetTrustLevel, CreateIUriBuilder::<Impl, OFFSET>, CreateInitializedIUriBuilder::<Impl, OFFSET>)
    }
}
pub trait IUriContainerImpl: Sized {
    fn GetIUri();
}
impl ::windows::core::RuntimeName for IUriContainer {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IUriContainer";
}
impl IUriContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriContainerImpl, const OFFSET: isize>() -> IUriContainerVtbl {
        unsafe extern "system" fn GetIUri<Impl: IUriContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIUri(::core::mem::transmute_copy(&ppiuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUriContainer>, ::windows::core::GetTrustLevel, GetIUri::<Impl, OFFSET>)
    }
}
pub trait IWinInetCacheHintsImpl: Sized {
    fn SetCacheExtension();
}
impl ::windows::core::RuntimeName for IWinInetCacheHints {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IWinInetCacheHints";
}
impl IWinInetCacheHintsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetCacheHintsImpl, const OFFSET: isize>() -> IWinInetCacheHintsVtbl {
        unsafe extern "system" fn SetCacheExtension<Impl: IWinInetCacheHintsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzext: super::super::super::Foundation::PWSTR, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCacheExtension(&*(&pwzext as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszcachefile as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), pcbcachefile, pdwwinineterror, pdwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWinInetCacheHints>, ::windows::core::GetTrustLevel, SetCacheExtension::<Impl, OFFSET>)
    }
}
pub trait IWinInetCacheHints2Impl: Sized + IWinInetCacheHintsImpl {
    fn SetCacheExtension2();
}
impl ::windows::core::RuntimeName for IWinInetCacheHints2 {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IWinInetCacheHints2";
}
impl IWinInetCacheHints2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetCacheHints2Impl, const OFFSET: isize>() -> IWinInetCacheHints2Vtbl {
        unsafe extern "system" fn SetCacheExtension2<Impl: IWinInetCacheHints2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzext: super::super::super::Foundation::PWSTR, pwzcachefile: super::super::super::Foundation::PWSTR, pcchcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCacheExtension2(&*(&pwzext as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pwzcachefile), pcchcachefile, ::core::mem::transmute_copy(&pdwwinineterror), ::core::mem::transmute_copy(&pdwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWinInetCacheHints2>, ::windows::core::GetTrustLevel, SetCacheExtension2::<Impl, OFFSET>)
    }
}
pub trait IWinInetFileStreamImpl: Sized {
    fn SetHandleForUnlock();
    fn SetDeleteFile();
}
impl ::windows::core::RuntimeName for IWinInetFileStream {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IWinInetFileStream";
}
impl IWinInetFileStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetFileStreamImpl, const OFFSET: isize>() -> IWinInetFileStreamVtbl {
        unsafe extern "system" fn SetHandleForUnlock<Impl: IWinInetFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwininetlockhandle: usize, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHandleForUnlock(hwininetlockhandle, dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeleteFile<Impl: IWinInetFileStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDeleteFile(dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWinInetFileStream>, ::windows::core::GetTrustLevel, SetHandleForUnlock::<Impl, OFFSET>, SetDeleteFile::<Impl, OFFSET>)
    }
}
pub trait IWinInetHttpInfoImpl: Sized + IWinInetInfoImpl {
    fn QueryInfo();
}
impl ::windows::core::RuntimeName for IWinInetHttpInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IWinInetHttpInfo";
}
impl IWinInetHttpInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetHttpInfoImpl, const OFFSET: isize>() -> IWinInetHttpInfoVtbl {
        unsafe extern "system" fn QueryInfo<Impl: IWinInetHttpInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32, pdwflags: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryInfo(dwoption, &*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), pcbbuf, pdwflags, pdwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWinInetHttpInfo>, ::windows::core::GetTrustLevel, QueryInfo::<Impl, OFFSET>)
    }
}
pub trait IWinInetHttpTimeoutsImpl: Sized {
    fn GetRequestTimeouts();
}
impl ::windows::core::RuntimeName for IWinInetHttpTimeouts {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IWinInetHttpTimeouts";
}
impl IWinInetHttpTimeoutsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetHttpTimeoutsImpl, const OFFSET: isize>() -> IWinInetHttpTimeoutsVtbl {
        unsafe extern "system" fn GetRequestTimeouts<Impl: IWinInetHttpTimeoutsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32, pdwsendtimeout: *mut u32, pdwreceivetimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestTimeouts(::core::mem::transmute_copy(&pdwconnecttimeout), ::core::mem::transmute_copy(&pdwsendtimeout), ::core::mem::transmute_copy(&pdwreceivetimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWinInetHttpTimeouts>, ::windows::core::GetTrustLevel, GetRequestTimeouts::<Impl, OFFSET>)
    }
}
pub trait IWinInetInfoImpl: Sized {
    fn QueryOption();
}
impl ::windows::core::RuntimeName for IWinInetInfo {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IWinInetInfo";
}
impl IWinInetInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWinInetInfoImpl, const OFFSET: isize>() -> IWinInetInfoVtbl {
        unsafe extern "system" fn QueryOption<Impl: IWinInetInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryOption(dwoption, &*(&pbuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), pcbbuf) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWinInetInfo>, ::windows::core::GetTrustLevel, QueryOption::<Impl, OFFSET>)
    }
}
pub trait IWindowForBindingUIImpl: Sized {
    fn GetWindow();
}
impl ::windows::core::RuntimeName for IWindowForBindingUI {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IWindowForBindingUI";
}
impl IWindowForBindingUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowForBindingUIImpl, const OFFSET: isize>() -> IWindowForBindingUIVtbl {
        unsafe extern "system" fn GetWindow<Impl: IWindowForBindingUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidreason: &::windows::core::GUID, phwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow(&*(&rguidreason as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowForBindingUI>, ::windows::core::GetTrustLevel, GetWindow::<Impl, OFFSET>)
    }
}
pub trait IWrappedProtocolImpl: Sized {
    fn GetWrapperCode();
}
impl ::windows::core::RuntimeName for IWrappedProtocol {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IWrappedProtocol";
}
impl IWrappedProtocolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWrappedProtocolImpl, const OFFSET: isize>() -> IWrappedProtocolVtbl {
        unsafe extern "system" fn GetWrapperCode<Impl: IWrappedProtocolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncode: *mut i32, dwreserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWrapperCode(::core::mem::transmute_copy(&pncode), dwreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWrappedProtocol>, ::windows::core::GetTrustLevel, GetWrapperCode::<Impl, OFFSET>)
    }
}
pub trait IZoneIdentifierImpl: Sized {
    fn GetId();
    fn SetId();
    fn Remove();
}
impl ::windows::core::RuntimeName for IZoneIdentifier {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IZoneIdentifier";
}
impl IZoneIdentifierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoneIdentifierImpl, const OFFSET: isize>() -> IZoneIdentifierVtbl {
        unsafe extern "system" fn GetId<Impl: IZoneIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&pdwzone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Impl: IZoneIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwzone: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetId(dwzone) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IZoneIdentifierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IZoneIdentifier>, ::windows::core::GetTrustLevel, GetId::<Impl, OFFSET>, SetId::<Impl, OFFSET>, Remove::<Impl, OFFSET>)
    }
}
pub trait IZoneIdentifier2Impl: Sized + IZoneIdentifierImpl {
    fn GetLastWriterPackageFamilyName();
    fn SetLastWriterPackageFamilyName();
    fn RemoveLastWriterPackageFamilyName();
    fn GetAppZoneId();
    fn SetAppZoneId();
    fn RemoveAppZoneId();
}
impl ::windows::core::RuntimeName for IZoneIdentifier2 {
    const NAME: &'static str = "Windows.Win32.System.Com.Urlmon.IZoneIdentifier2";
}
impl IZoneIdentifier2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IZoneIdentifier2Impl, const OFFSET: isize>() -> IZoneIdentifier2Vtbl {
        unsafe extern "system" fn GetLastWriterPackageFamilyName<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastWriterPackageFamilyName(::core::mem::transmute_copy(&packagefamilyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastWriterPackageFamilyName<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLastWriterPackageFamilyName(&*(&packagefamilyname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLastWriterPackageFamilyName<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveLastWriterPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppZoneId<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppZoneId(::core::mem::transmute_copy(&zone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppZoneId<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zone: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAppZoneId(zone) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAppZoneId<Impl: IZoneIdentifier2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAppZoneId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IZoneIdentifier2>,
            ::windows::core::GetTrustLevel,
            GetLastWriterPackageFamilyName::<Impl, OFFSET>,
            SetLastWriterPackageFamilyName::<Impl, OFFSET>,
            RemoveLastWriterPackageFamilyName::<Impl, OFFSET>,
            GetAppZoneId::<Impl, OFFSET>,
            SetAppZoneId::<Impl, OFFSET>,
            RemoveAppZoneId::<Impl, OFFSET>,
        )
    }
}
