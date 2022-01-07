#[cfg(feature = "Win32_System_Com")]
pub trait DRendezvousSessionEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DRendezvousSessionEvents {
    const NAME: &'static str = "Windows.Win32.System.RemoteAssistance.DRendezvousSessionEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl DRendezvousSessionEventsVtbl {
    pub const fn new<Impl: DRendezvousSessionEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> DRendezvousSessionEventsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<DRendezvousSessionEvents>, base.5)
    }
}
pub trait IRendezvousApplicationImpl: Sized {
    fn SetRendezvousSession();
}
impl ::windows::core::RuntimeName for IRendezvousApplication {
    const NAME: &'static str = "Windows.Win32.System.RemoteAssistance.IRendezvousApplication";
}
impl IRendezvousApplicationVtbl {
    pub const fn new<Impl: IRendezvousApplicationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRendezvousApplicationVtbl {
        unsafe extern "system" fn SetRendezvousSession<Impl: IRendezvousApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRendezvousSession(&*(&prendezvoussession as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRendezvousApplication>, base.5, SetRendezvousSession::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRendezvousSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRendezvousSessionVtbl {
        unsafe extern "system" fn State<Impl: IRendezvousSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&psessionstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUser<Impl: IRendezvousSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoteUser(::core::mem::transmute_copy(&bstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRendezvousSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flags(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendContextData<Impl: IRendezvousSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SendContextData(&*(&bstrdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IRendezvousSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, bstrappdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminate(hr, &*(&bstrappdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRendezvousSession>, base.5, State::<Impl, OFFSET>, RemoteUser::<Impl, OFFSET>, Flags::<Impl, OFFSET>, SendContextData::<Impl, OFFSET>, Terminate::<Impl, OFFSET>)
    }
}
