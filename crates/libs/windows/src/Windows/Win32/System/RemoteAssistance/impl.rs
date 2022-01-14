#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DRendezvousSessionEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DRendezvousSessionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DRendezvousSessionEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DRendezvousSessionEvents_Vtbl {
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DRendezvousSessionEvents as ::windows::core::Interface>::IID
    }
}
pub trait IRendezvousApplication_Impl: Sized {
    fn SetRendezvousSession(&mut self, prendezvoussession: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IRendezvousApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRendezvousApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRendezvousApplication_Vtbl {
        unsafe extern "system" fn SetRendezvousSession<Impl: IRendezvousApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IRendezvousSession_Impl: Sized {
    fn State(&mut self) -> ::windows::core::Result<RENDEZVOUS_SESSION_STATE>;
    fn RemoteUser(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Flags(&mut self) -> ::windows::core::Result<i32>;
    fn SendContextData(&mut self, bstrdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Terminate(&mut self, hr: ::windows::core::HRESULT, bstrappdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRendezvousSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRendezvousSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRendezvousSession_Vtbl {
        unsafe extern "system" fn State<Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *psessionstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUser<Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteUser() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendContextData<Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendContextData(::core::mem::transmute_copy(&bstrdata)).into()
        }
        unsafe extern "system" fn Terminate<Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, bstrappdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
