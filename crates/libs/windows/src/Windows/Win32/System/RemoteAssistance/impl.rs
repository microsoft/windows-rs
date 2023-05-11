#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DRendezvousSessionEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for DRendezvousSessionEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DRendezvousSessionEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DRendezvousSessionEvents_Impl, const OFFSET: isize>() -> DRendezvousSessionEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<DRendezvousSessionEvents as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`, `\"implement\"`*"]
pub trait IRendezvousApplication_Impl: Sized {
    fn SetRendezvousSession(&self, prendezvoussession: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IRendezvousApplication {}
impl IRendezvousApplication_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousApplication_Impl, const OFFSET: isize>() -> IRendezvousApplication_Vtbl {
        unsafe extern "system" fn SetRendezvousSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRendezvousSession(::windows_core::from_raw_borrowed(&prendezvoussession)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRendezvousSession: SetRendezvousSession::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRendezvousApplication as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteAssistance\"`, `\"implement\"`*"]
pub trait IRendezvousSession_Impl: Sized {
    fn State(&self) -> ::windows_core::Result<RENDEZVOUS_SESSION_STATE>;
    fn RemoteUser(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Flags(&self) -> ::windows_core::Result<i32>;
    fn SendContextData(&self, bstrdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Terminate(&self, hr: ::windows_core::HRESULT, bstrappdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IRendezvousSession {}
impl IRendezvousSession_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>() -> IRendezvousSession_Vtbl {
        unsafe extern "system" fn State<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psessionstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoteUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Flags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendContextData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendContextData(::core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, bstrappdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate(::core::mem::transmute_copy(&hr), ::core::mem::transmute(&bstrappdata)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            State: State::<Identity, Impl, OFFSET>,
            RemoteUser: RemoteUser::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SendContextData: SendContextData::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRendezvousSession as ::windows_core::ComInterface>::IID
    }
}
