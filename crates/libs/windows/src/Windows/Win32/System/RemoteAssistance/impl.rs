#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DRendezvousSessionEventsImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DRendezvousSessionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DRendezvousSessionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DRendezvousSessionEventsVtbl {
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DRendezvousSessionEvents as ::windows::core::Interface>::IID
    }
}
pub trait IRendezvousApplicationImpl: Sized {
    fn SetRendezvousSession(&mut self, prendezvoussession: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IRendezvousApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRendezvousApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRendezvousApplicationVtbl {
        unsafe extern "system" fn SetRendezvousSession<Impl: IRendezvousApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRendezvousSession(::core::mem::transmute(&prendezvoussession)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetRendezvousSession: SetRendezvousSession::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRendezvousApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRendezvousSessionImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<RENDEZVOUS_SESSION_STATE>;
    fn RemoteUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn SendContextData(&mut self, bstrdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Terminate(&mut self, hr: ::windows::core::HRESULT, bstrappdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRendezvousSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRendezvousSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRendezvousSessionVtbl {
        unsafe extern "system" fn State<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *psessionstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUser<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteUser() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendContextData<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendContextData(::core::mem::transmute_copy(&bstrdata)).into()
        }
        unsafe extern "system" fn Terminate<Impl: IRendezvousSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, bstrappdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&bstrappdata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            RemoteUser: RemoteUser::<Impl, IMPL_OFFSET>,
            Flags: Flags::<Impl, IMPL_OFFSET>,
            SendContextData: SendContextData::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRendezvousSession as ::windows::core::Interface>::IID
    }
}
