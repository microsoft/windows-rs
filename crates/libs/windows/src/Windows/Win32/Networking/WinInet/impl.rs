#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDialBrandingImpl: Sized {
    fn Initialize(&mut self, pwzconnectoid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetBitmap(&mut self, dwindex: u32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDialBrandingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialBrandingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDialBrandingVtbl {
        unsafe extern "system" fn Initialize<Impl: IDialBrandingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzconnectoid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pwzconnectoid)).into()
        }
        unsafe extern "system" fn GetBitmap<Impl: IDialBrandingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitmap(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *phbitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetBitmap: GetBitmap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialBranding as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDialEngineImpl: Sized {
    fn Initialize(&mut self, pwzconnectoid: super::super::Foundation::PWSTR, pides: ::core::option::Option<IDialEventSink>) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR, dwbufsize: u32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Dial(&mut self) -> ::windows::core::Result<()>;
    fn HangUp(&mut self) -> ::windows::core::Result<()>;
    fn GetConnectedState(&mut self) -> ::windows::core::Result<u32>;
    fn GetConnectHandle(&mut self) -> ::windows::core::Result<usize>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDialEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDialEngineVtbl {
        unsafe extern "system" fn Initialize<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzconnectoid: super::super::Foundation::PWSTR, pides: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pwzconnectoid), ::core::mem::transmute(&pides)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&pwzproperty), ::core::mem::transmute_copy(&pwzvalue), ::core::mem::transmute_copy(&dwbufsize)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&pwzproperty), ::core::mem::transmute_copy(&pwzvalue)).into()
        }
        unsafe extern "system" fn Dial<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dial().into()
        }
        unsafe extern "system" fn HangUp<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HangUp().into()
        }
        unsafe extern "system" fn GetConnectedState<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectedState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectHandle<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Dial: Dial::<Impl, IMPL_OFFSET>,
            HangUp: HangUp::<Impl, IMPL_OFFSET>,
            GetConnectedState: GetConnectedState::<Impl, IMPL_OFFSET>,
            GetConnectHandle: GetConnectHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialEngine as ::windows::core::Interface>::IID
    }
}
pub trait IDialEventSinkImpl: Sized {
    fn OnEvent(&mut self, dwevent: u32, dwstatus: u32) -> ::windows::core::Result<()>;
}
impl IDialEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDialEventSinkVtbl {
        unsafe extern "system" fn OnEvent<Impl: IDialEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwevent: u32, dwstatus: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEvent(::core::mem::transmute_copy(&dwevent), ::core::mem::transmute_copy(&dwstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnEvent: OnEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProofOfPossessionCookieInfoManagerImpl: Sized {
    fn GetCookieInfoForUri(&mut self, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProofOfPossessionCookieInfoManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProofOfPossessionCookieInfoManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProofOfPossessionCookieInfoManagerVtbl {
        unsafe extern "system" fn GetCookieInfoForUri<Impl: IProofOfPossessionCookieInfoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCookieInfoForUri(::core::mem::transmute_copy(&uri), ::core::mem::transmute_copy(&cookieinfocount), ::core::mem::transmute_copy(&cookieinfo)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCookieInfoForUri: GetCookieInfoForUri::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProofOfPossessionCookieInfoManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProofOfPossessionCookieInfoManager2Impl: Sized {
    fn GetCookieInfoWithUriForAccount(&mut self, webaccount: ::core::option::Option<::windows::core::IInspectable>, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProofOfPossessionCookieInfoManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProofOfPossessionCookieInfoManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProofOfPossessionCookieInfoManager2Vtbl {
        unsafe extern "system" fn GetCookieInfoWithUriForAccount<Impl: IProofOfPossessionCookieInfoManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCookieInfoWithUriForAccount(::core::mem::transmute(&webaccount), ::core::mem::transmute_copy(&uri), ::core::mem::transmute_copy(&cookieinfocount), ::core::mem::transmute_copy(&cookieinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCookieInfoWithUriForAccount: GetCookieInfoWithUriForAccount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProofOfPossessionCookieInfoManager2 as ::windows::core::Interface>::IID
    }
}
