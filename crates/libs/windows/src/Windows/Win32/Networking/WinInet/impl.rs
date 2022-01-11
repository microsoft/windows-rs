#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDialBrandingImpl: Sized {
    fn Initialize();
    fn GetBitmap();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IDialBrandingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialBrandingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDialBrandingVtbl {
        unsafe extern "system" fn Initialize<Impl: IDialBrandingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzconnectoid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitmap<Impl: IDialBrandingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Initialize();
    fn GetProperty();
    fn SetProperty();
    fn Dial();
    fn HangUp();
    fn GetConnectedState();
    fn GetConnectHandle();
}
#[cfg(feature = "Win32_Foundation")]
impl IDialEngineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialEngineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDialEngineVtbl {
        unsafe extern "system" fn Initialize<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzconnectoid: super::super::Foundation::PWSTR, pides: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Dial<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HangUp<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectedState<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectHandle<Impl: IDialEngineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn OnEvent();
}
impl IDialEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDialEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDialEventSinkVtbl {
        unsafe extern "system" fn OnEvent<Impl: IDialEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwevent: u32, dwstatus: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnEvent: OnEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDialEventSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProofOfPossessionCookieInfoManagerImpl: Sized {
    fn GetCookieInfoForUri();
}
#[cfg(feature = "Win32_Foundation")]
impl IProofOfPossessionCookieInfoManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProofOfPossessionCookieInfoManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProofOfPossessionCookieInfoManagerVtbl {
        unsafe extern "system" fn GetCookieInfoForUri<Impl: IProofOfPossessionCookieInfoManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCookieInfoForUri: GetCookieInfoForUri::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProofOfPossessionCookieInfoManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProofOfPossessionCookieInfoManager2Impl: Sized {
    fn GetCookieInfoWithUriForAccount();
}
#[cfg(feature = "Win32_Foundation")]
impl IProofOfPossessionCookieInfoManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProofOfPossessionCookieInfoManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProofOfPossessionCookieInfoManager2Vtbl {
        unsafe extern "system" fn GetCookieInfoWithUriForAccount<Impl: IProofOfPossessionCookieInfoManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
