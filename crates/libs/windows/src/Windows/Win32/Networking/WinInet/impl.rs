pub trait IDialBrandingImpl: Sized {
    fn Initialize();
    fn GetBitmap();
}
impl ::windows::core::RuntimeName for IDialBranding {
    const NAME: &'static str = "Windows.Win32.Networking.WinInet.IDialBranding";
}
impl IDialBrandingVtbl {
    pub const fn new<Impl: IDialBrandingImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDialBrandingVtbl {
        unsafe extern "system" fn Initialize<Impl: IDialBrandingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzconnectoid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pwzconnectoid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitmap<Impl: IDialBrandingImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBitmap(dwindex, ::core::mem::transmute_copy(&phbitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDialBranding>, base.5, Initialize::<Impl, OFFSET>, GetBitmap::<Impl, OFFSET>)
    }
}
pub trait IDialEngineImpl: Sized {
    fn Initialize();
    fn GetProperty();
    fn SetProperty();
    fn Dial();
    fn HangUp();
    fn GetConnectedState();
    fn GetConnectHandle();
}
impl ::windows::core::RuntimeName for IDialEngine {
    const NAME: &'static str = "Windows.Win32.Networking.WinInet.IDialEngine";
}
impl IDialEngineVtbl {
    pub const fn new<Impl: IDialEngineImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDialEngineVtbl {
        unsafe extern "system" fn Initialize<Impl: IDialEngineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzconnectoid: super::super::Foundation::PWSTR, pides: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pwzconnectoid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pides as *const <IDialEventSink as ::windows::core::Abi>::Abi as *const <IDialEventSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IDialEngineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR, dwbufsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&pwzproperty as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwzvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwbufsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IDialEngineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzproperty: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&pwzproperty as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwzvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dial<Impl: IDialEngineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HangUp<Impl: IDialEngineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HangUp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectedState<Impl: IDialEngineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectedState(::core::mem::transmute_copy(&pdwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectHandle<Impl: IDialEngineImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwhandle: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConnectHandle(::core::mem::transmute_copy(&pdwhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDialEngine>, base.5, Initialize::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>, Dial::<Impl, OFFSET>, HangUp::<Impl, OFFSET>, GetConnectedState::<Impl, OFFSET>, GetConnectHandle::<Impl, OFFSET>)
    }
}
pub trait IDialEventSinkImpl: Sized {
    fn OnEvent();
}
impl ::windows::core::RuntimeName for IDialEventSink {
    const NAME: &'static str = "Windows.Win32.Networking.WinInet.IDialEventSink";
}
impl IDialEventSinkVtbl {
    pub const fn new<Impl: IDialEventSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDialEventSinkVtbl {
        unsafe extern "system" fn OnEvent<Impl: IDialEventSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwevent: u32, dwstatus: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnEvent(dwevent, dwstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDialEventSink>, base.5, OnEvent::<Impl, OFFSET>)
    }
}
pub trait IProofOfPossessionCookieInfoManagerImpl: Sized {
    fn GetCookieInfoForUri();
}
impl ::windows::core::RuntimeName for IProofOfPossessionCookieInfoManager {
    const NAME: &'static str = "Windows.Win32.Networking.WinInet.IProofOfPossessionCookieInfoManager";
}
impl IProofOfPossessionCookieInfoManagerVtbl {
    pub const fn new<Impl: IProofOfPossessionCookieInfoManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProofOfPossessionCookieInfoManagerVtbl {
        unsafe extern "system" fn GetCookieInfoForUri<Impl: IProofOfPossessionCookieInfoManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCookieInfoForUri(&*(&uri as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cookieinfocount), ::core::mem::transmute_copy(&cookieinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProofOfPossessionCookieInfoManager>, base.5, GetCookieInfoForUri::<Impl, OFFSET>)
    }
}
pub trait IProofOfPossessionCookieInfoManager2Impl: Sized {
    fn GetCookieInfoWithUriForAccount();
}
impl ::windows::core::RuntimeName for IProofOfPossessionCookieInfoManager2 {
    const NAME: &'static str = "Windows.Win32.Networking.WinInet.IProofOfPossessionCookieInfoManager2";
}
impl IProofOfPossessionCookieInfoManager2Vtbl {
    pub const fn new<Impl: IProofOfPossessionCookieInfoManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProofOfPossessionCookieInfoManager2Vtbl {
        unsafe extern "system" fn GetCookieInfoWithUriForAccount<Impl: IProofOfPossessionCookieInfoManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, uri: super::super::Foundation::PWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCookieInfoWithUriForAccount(&*(&webaccount as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&uri as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cookieinfocount), ::core::mem::transmute_copy(&cookieinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProofOfPossessionCookieInfoManager2>, base.5, GetCookieInfoWithUriForAccount::<Impl, OFFSET>)
    }
}
