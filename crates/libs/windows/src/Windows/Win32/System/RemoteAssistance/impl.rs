#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DRendezvousSessionEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for DRendezvousSessionEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DRendezvousSessionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: DRendezvousSessionEvents_Impl, const OFFSET: isize>() -> DRendezvousSessionEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DRendezvousSessionEvents as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IRendezvousApplication_Impl: Sized {
    fn SetRendezvousSession(&self, prendezvoussession: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRendezvousApplication {}
impl IRendezvousApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousApplication_Impl, const OFFSET: isize>() -> IRendezvousApplication_Vtbl {
        unsafe extern "system" fn SetRendezvousSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRendezvousSession(::core::mem::transmute(&prendezvoussession)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetRendezvousSession: SetRendezvousSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRendezvousApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRendezvousSession_Impl: Sized {
    fn State(&self) -> ::windows::core::Result<RENDEZVOUS_SESSION_STATE>;
    fn RemoteUser(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Flags(&self) -> ::windows::core::Result<i32>;
    fn SendContextData(&self, bstrdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Terminate(&self, hr: ::windows::core::HRESULT, bstrappdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRendezvousSession {}
#[cfg(feature = "Win32_Foundation")]
impl IRendezvousSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>() -> IRendezvousSession_Vtbl {
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psessionstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoteUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Flags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendContextData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendContextData(::core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, bstrappdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate(::core::mem::transmute_copy(&hr), ::core::mem::transmute(&bstrappdata)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            State: State::<Identity, Impl, OFFSET>,
            RemoteUser: RemoteUser::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SendContextData: SendContextData::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRendezvousSession as ::windows::core::Interface>::IID
    }
}
