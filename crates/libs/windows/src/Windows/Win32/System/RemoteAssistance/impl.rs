#[cfg(feature = "Win32_System_Com")]
pub trait DRendezvousSessionEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DRendezvousSessionEvents {
    const NAME: &'static str = "Windows.Win32.System.RemoteAssistance.DRendezvousSessionEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl DRendezvousSessionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DRendezvousSessionEventsImpl, const OFFSET: isize>() -> DRendezvousSessionEventsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<DRendezvousSessionEvents>, ::windows::core::GetTrustLevel)
    }
}
pub trait IRendezvousApplicationImpl: Sized {
    fn SetRendezvousSession();
}
impl ::windows::core::RuntimeName for IRendezvousApplication {
    const NAME: &'static str = "Windows.Win32.System.RemoteAssistance.IRendezvousApplication";
}
impl IRendezvousApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRendezvousApplicationImpl, const OFFSET: isize>() -> IRendezvousApplicationVtbl {
        unsafe extern "system" fn SetRendezvousSession<Impl: IRendezvousApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRendezvousSession(&*(&prendezvoussession as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRendezvousApplication>, ::windows::core::GetTrustLevel, SetRendezvousSession::<Impl, OFFSET>)
    }
}
pub trait IRendezvousSessionImpl: Sized {
    fn State();
    fn RemoteUser();
    fn Flags();
    fn SendContextData();
    fn Terminate();
}
impl ::windows::core::RuntimeName for IRendezvousSession {
    const NAME: &'static str = "Windows.Win32.System.RemoteAssistance.IRendezvousSession";
}
impl IRendezvousSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRendezvousSessionImpl, const OFFSET: isize>() -> IRendezvousSessionVtbl {
        unsafe extern "system" fn State<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&psessionstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUser<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteUser(::core::mem::transmute_copy(&bstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendContextData<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendContextData(&*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, bstrappdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminate(hr, &*(&bstrappdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRendezvousSession>, ::windows::core::GetTrustLevel, State::<Impl, OFFSET>, RemoteUser::<Impl, OFFSET>, Flags::<Impl, OFFSET>, SendContextData::<Impl, OFFSET>, Terminate::<Impl, OFFSET>)
    }
}
