#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDialBranding_Impl: Sized {
    fn Initialize(&mut self, pwzconnectoid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetBitmap(&mut self, dwindex: u32) -> ::windows::core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDialBranding_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialBranding_Impl, const OFFSET: isize>() -> IDialBranding_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDialBranding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzconnectoid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pwzconnectoid)).into()
        }
        unsafe extern "system" fn GetBitmap<Identity: ::windows::core::IUnknownImpl, Impl: IDialBranding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBitmap(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *phbitmap = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetBitmap: GetBitmap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialBranding as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDialEngine_Impl: Sized {
    fn Initialize(&mut self, pwzconnectoid: super::super::Foundation::PWSTR, pides: &::core::option::Option<IDialEventSink>) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR, dwbufsize: u32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Dial(&mut self) -> ::windows::core::Result<()>;
    fn HangUp(&mut self) -> ::windows::core::Result<()>;
    fn GetConnectedState(&mut self) -> ::windows::core::Result<u32>;
    fn GetConnectHandle(&mut self) -> ::windows::core::Result<usize>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDialEngine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngine_Impl, const OFFSET: isize>() -> IDialEngine_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzconnectoid: super::super::Foundation::PWSTR, pides: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pwzconnectoid), ::core::mem::transmute(&pides)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&pwzproperty), ::core::mem::transmute_copy(&pwzvalue), ::core::mem::transmute_copy(&dwbufsize)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&pwzproperty), ::core::mem::transmute_copy(&pwzvalue)).into()
        }
        unsafe extern "system" fn Dial<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Dial().into()
        }
        unsafe extern "system" fn HangUp<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HangUp().into()
        }
        unsafe extern "system" fn GetConnectedState<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnectedState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnectHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Dial: Dial::<Identity, Impl, OFFSET>,
            HangUp: HangUp::<Identity, Impl, OFFSET>,
            GetConnectedState: GetConnectedState::<Identity, Impl, OFFSET>,
            GetConnectHandle: GetConnectHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialEngine as ::windows::core::Interface>::IID
    }
}
pub trait IDialEventSink_Impl: Sized {
    fn OnEvent(&mut self, dwevent: u32, dwstatus: u32) -> ::windows::core::Result<()>;
}
impl IDialEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialEventSink_Impl, const OFFSET: isize>() -> IDialEventSink_Vtbl {
        unsafe extern "system" fn OnEvent<Identity: ::windows::core::IUnknownImpl, Impl: IDialEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwevent: u32, dwstatus: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEvent(::core::mem::transmute_copy(&dwevent), ::core::mem::transmute_copy(&dwstatus)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnEvent: OnEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProofOfPossessionCookieInfoManager_Impl: Sized {
    fn GetCookieInfoForUri(&mut self, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProofOfPossessionCookieInfoManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProofOfPossessionCookieInfoManager_Impl, const OFFSET: isize>() -> IProofOfPossessionCookieInfoManager_Vtbl {
        unsafe extern "system" fn GetCookieInfoForUri<Identity: ::windows::core::IUnknownImpl, Impl: IProofOfPossessionCookieInfoManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCookieInfoForUri(::core::mem::transmute_copy(&uri), ::core::mem::transmute_copy(&cookieinfocount), ::core::mem::transmute_copy(&cookieinfo)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetCookieInfoForUri: GetCookieInfoForUri::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProofOfPossessionCookieInfoManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProofOfPossessionCookieInfoManager2_Impl: Sized {
    fn GetCookieInfoWithUriForAccount(&mut self, webaccount: &::core::option::Option<::windows::core::IInspectable>, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProofOfPossessionCookieInfoManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProofOfPossessionCookieInfoManager2_Impl, const OFFSET: isize>() -> IProofOfPossessionCookieInfoManager2_Vtbl {
        unsafe extern "system" fn GetCookieInfoWithUriForAccount<Identity: ::windows::core::IUnknownImpl, Impl: IProofOfPossessionCookieInfoManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCookieInfoWithUriForAccount(::core::mem::transmute(&webaccount), ::core::mem::transmute_copy(&uri), ::core::mem::transmute_copy(&cookieinfocount), ::core::mem::transmute_copy(&cookieinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCookieInfoWithUriForAccount: GetCookieInfoWithUriForAccount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProofOfPossessionCookieInfoManager2 as ::windows::core::Interface>::IID
    }
}
