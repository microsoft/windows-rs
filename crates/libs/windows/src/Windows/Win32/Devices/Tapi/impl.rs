#[cfg(feature = "Win32_System_Com")]
pub trait IEnumACDGroupImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITACDGroup>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumACDGroup>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumACDGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumACDGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumACDGroupVtbl {
        unsafe extern "system" fn Next<Impl: IEnumACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumACDGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAddressImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITAddress>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumAddress>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAddressVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgentImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITAgent>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumAgent>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAgentVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgentHandlerImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITAgentHandler>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumAgentHandler>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgentHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAgentHandlerVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAgentHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgentSessionImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITAgentSession>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumAgentSession>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgentSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAgentSessionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAgentSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumBstrImpl: Sized {
    fn Next(&mut self, celt: u32, ppstrings: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBstr>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumBstrVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBstrImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBstrVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBstrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstrings: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstrings), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBstrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBstrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBstrImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBstr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCallImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITCallInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCall>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCallVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCall as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCallHubImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITCallHub>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCallHub>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCallHubVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallHubImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCallHubVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCallHub as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCallingCardImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITCallingCard>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCallingCard>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCallingCardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallingCardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCallingCardVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCallingCard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumDialableAddrsImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDialableAddrs>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumDialableAddrsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDialableAddrsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDialableAddrsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDialableAddrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDialableAddrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDialableAddrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDialableAddrsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDialableAddrs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumDirectoryImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITDirectory>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDirectory>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumDirectoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDirectoryVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDirectory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumDirectoryObjectImpl: Sized {
    fn Next(&mut self, celt: u32, pval: *mut ::core::option::Option<ITDirectoryObject>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDirectoryObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumDirectoryObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDirectoryObjectVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pval: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDirectoryObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumLocationImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITLocationInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumLocation>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumLocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumLocationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumLocationVtbl {
        unsafe extern "system" fn Next<Impl: IEnumLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumLocation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMcastScopeImpl: Sized {
    fn Next(&mut self, celt: u32, ppscopes: *mut ::core::option::Option<IMcastScope>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumMcastScope>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMcastScopeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMcastScopeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumMcastScopeVtbl {
        unsafe extern "system" fn Next<Impl: IEnumMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppscopes: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppscopes), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMcastScope as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPhoneImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITPhone>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumPhone>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPhoneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPhoneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPhoneVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPhone as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPluggableSuperclassInfoImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITPluggableTerminalSuperclassInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumPluggableSuperclassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPluggableSuperclassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableSuperclassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPluggableSuperclassInfoVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPluggableSuperclassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPluggableTerminalClassInfoImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITPluggableTerminalClassInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPluggableTerminalClassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableTerminalClassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPluggableTerminalClassInfoVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPluggableTerminalClassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumQueueImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITQueue>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumQueue>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumQueueVtbl {
        unsafe extern "system" fn Next<Impl: IEnumQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumStreamImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITStream>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumStreamVtbl {
        unsafe extern "system" fn Next<Impl: IEnumStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumSubStreamImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITSubStream>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSubStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumSubStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSubStreamVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSubStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumTerminalImpl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITTerminal>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTerminal>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTerminalVtbl {
        unsafe extern "system" fn Next<Impl: IEnumTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTerminal as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTerminalClassImpl: Sized {
    fn Next(&mut self, celt: u32, pelements: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTerminalClass>;
}
impl IEnumTerminalClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTerminalClassVtbl {
        unsafe extern "system" fn Next<Impl: IEnumTerminalClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pelements: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTerminalClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTerminalClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumTerminalClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTerminalClass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastAddressAllocationImpl: Sized + IDispatchImpl {
    fn Scopes(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateScopes(&mut self) -> ::windows::core::Result<IEnumMcastScope>;
    fn RequestAddress(&mut self, pscope: ::core::option::Option<IMcastScope>, leasestarttime: f64, leasestoptime: f64, numaddresses: i32) -> ::windows::core::Result<IMcastLeaseInfo>;
    fn RenewAddress(&mut self, lreserved: i32, prenewrequest: ::core::option::Option<IMcastLeaseInfo>) -> ::windows::core::Result<IMcastLeaseInfo>;
    fn ReleaseAddress(&mut self, preleaserequest: ::core::option::Option<IMcastLeaseInfo>) -> ::windows::core::Result<()>;
    fn CreateLeaseInfo(&mut self, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const super::super::Foundation::PWSTR, prequestid: super::super::Foundation::PWSTR, pserveraddress: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMcastLeaseInfo>;
    fn CreateLeaseInfoFromVariant(&mut self, leasestarttime: f64, leasestoptime: f64, vaddresses: super::super::System::Com::VARIANT, prequestid: super::super::Foundation::BSTR, pserveraddress: super::super::Foundation::BSTR) -> ::windows::core::Result<IMcastLeaseInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastAddressAllocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMcastAddressAllocationVtbl {
        unsafe extern "system" fn Scopes<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scopes() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateScopes<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenummcastscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateScopes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenummcastscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAddress<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscope: ::windows::core::RawPtr, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAddress(::core::mem::transmute(&pscope), ::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute_copy(&numaddresses)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppleaseresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewAddress<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lreserved: i32, prenewrequest: ::windows::core::RawPtr, pprenewresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewAddress(::core::mem::transmute_copy(&lreserved), ::core::mem::transmute(&prenewrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprenewresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseAddress<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preleaserequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseAddress(::core::mem::transmute(&preleaserequest)).into()
        }
        unsafe extern "system" fn CreateLeaseInfo<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const super::super::Foundation::PWSTR, prequestid: super::super::Foundation::PWSTR, pserveraddress: super::super::Foundation::PWSTR, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLeaseInfo(::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute_copy(&dwnumaddresses), ::core::mem::transmute_copy(&ppaddresses), ::core::mem::transmute_copy(&prequestid), ::core::mem::transmute_copy(&pserveraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreleaserequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLeaseInfoFromVariant<Impl: IMcastAddressAllocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, vaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, prequestid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pserveraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLeaseInfoFromVariant(::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute_copy(&vaddresses), ::core::mem::transmute_copy(&prequestid), ::core::mem::transmute_copy(&pserveraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreleaserequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Scopes: Scopes::<Impl, IMPL_OFFSET>,
            EnumerateScopes: EnumerateScopes::<Impl, IMPL_OFFSET>,
            RequestAddress: RequestAddress::<Impl, IMPL_OFFSET>,
            RenewAddress: RenewAddress::<Impl, IMPL_OFFSET>,
            ReleaseAddress: ReleaseAddress::<Impl, IMPL_OFFSET>,
            CreateLeaseInfo: CreateLeaseInfo::<Impl, IMPL_OFFSET>,
            CreateLeaseInfoFromVariant: CreateLeaseInfoFromVariant::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastAddressAllocation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastLeaseInfoImpl: Sized + IDispatchImpl {
    fn RequestID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LeaseStartTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetLeaseStartTime(&mut self, time: f64) -> ::windows::core::Result<()>;
    fn LeaseStopTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetLeaseStopTime(&mut self, time: f64) -> ::windows::core::Result<()>;
    fn AddressCount(&mut self) -> ::windows::core::Result<i32>;
    fn ServerAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TTL(&mut self) -> ::windows::core::Result<i32>;
    fn Addresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateAddresses(&mut self) -> ::windows::core::Result<IEnumBstr>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastLeaseInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMcastLeaseInfoVtbl {
        unsafe extern "system" fn RequestID<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprequestid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestID() {
                ::core::result::Result::Ok(ok__) => {
                    *pprequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseStartTime<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeaseStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStartTime<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeaseStartTime(::core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn LeaseStopTime<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeaseStopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStopTime<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeaseStopTime(::core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn AddressCount<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerAddress<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Addresses<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: IMcastLeaseInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddresses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RequestID: RequestID::<Impl, IMPL_OFFSET>,
            LeaseStartTime: LeaseStartTime::<Impl, IMPL_OFFSET>,
            SetLeaseStartTime: SetLeaseStartTime::<Impl, IMPL_OFFSET>,
            LeaseStopTime: LeaseStopTime::<Impl, IMPL_OFFSET>,
            SetLeaseStopTime: SetLeaseStopTime::<Impl, IMPL_OFFSET>,
            AddressCount: AddressCount::<Impl, IMPL_OFFSET>,
            ServerAddress: ServerAddress::<Impl, IMPL_OFFSET>,
            TTL: TTL::<Impl, IMPL_OFFSET>,
            Addresses: Addresses::<Impl, IMPL_OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastLeaseInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastScopeImpl: Sized + IDispatchImpl {
    fn ScopeID(&mut self) -> ::windows::core::Result<i32>;
    fn ServerID(&mut self) -> ::windows::core::Result<i32>;
    fn InterfaceID(&mut self) -> ::windows::core::Result<i32>;
    fn ScopeDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TTL(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastScopeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastScopeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMcastScopeVtbl {
        unsafe extern "system" fn ScopeID<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerID<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeDescription<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Impl: IMcastScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ScopeID: ScopeID::<Impl, IMPL_OFFSET>,
            ServerID: ServerID::<Impl, IMPL_OFFSET>,
            InterfaceID: InterfaceID::<Impl, IMPL_OFFSET>,
            ScopeDescription: ScopeDescription::<Impl, IMPL_OFFSET>,
            TTL: TTL::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastScope as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITACDGroupImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnumerateQueues(&mut self) -> ::windows::core::Result<IEnumQueue>;
    fn Queues(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITACDGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITACDGroupVtbl {
        unsafe extern "system" fn Name<Impl: ITACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateQueues<Impl: ITACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Queues<Impl: ITACDGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Queues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            EnumerateQueues: EnumerateQueues::<Impl, IMPL_OFFSET>,
            Queues: Queues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITACDGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITACDGroupEventImpl: Sized + IDispatchImpl {
    fn Group(&mut self) -> ::windows::core::Result<ITACDGroup>;
    fn Event(&mut self) -> ::windows::core::Result<ACDGROUP_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITACDGroupEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroupEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITACDGroupEventVtbl {
        unsafe extern "system" fn Group<Impl: ITACDGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITACDGroupEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDGROUP_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Group: Group::<Impl, IMPL_OFFSET>, Event: Event::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITACDGroupEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait ITAMMediaFormatImpl: Sized {
    fn MediaFormat(&mut self) -> ::windows::core::Result<*mut super::super::Media::DirectShow::AM_MEDIA_TYPE>;
    fn SetMediaFormat(&mut self, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ITAMMediaFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAMMediaFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAMMediaFormatVtbl {
        unsafe extern "system" fn MediaFormat<Impl: ITAMMediaFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaFormat<Impl: ITAMMediaFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaFormat(::core::mem::transmute_copy(&pmt)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MediaFormat: MediaFormat::<Impl, IMPL_OFFSET>,
            SetMediaFormat: SetMediaFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAMMediaFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITASRTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITASRTerminalEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITASRTerminalEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITASRTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITASRTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITASRTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITASRTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITASRTerminalEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressImpl: Sized + IDispatchImpl {
    fn State(&mut self) -> ::windows::core::Result<ADDRESS_STATE>;
    fn AddressName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TAPIObject(&mut self) -> ::windows::core::Result<ITTAPI>;
    fn CreateCall(&mut self, pdestaddress: super::super::Foundation::BSTR, laddresstype: i32, lmediatypes: i32) -> ::windows::core::Result<ITBasicCallControl>;
    fn Calls(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCalls(&mut self) -> ::windows::core::Result<IEnumCall>;
    fn DialableAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateForwardInfoObject(&mut self) -> ::windows::core::Result<ITForwardInformation>;
    fn Forward(&mut self, pforwardinfo: ::core::option::Option<ITForwardInformation>, pcall: ::core::option::Option<ITBasicCallControl>) -> ::windows::core::Result<()>;
    fn CurrentForwardInfo(&mut self) -> ::windows::core::Result<ITForwardInformation>;
    fn SetMessageWaiting(&mut self, fmessagewaiting: i16) -> ::windows::core::Result<()>;
    fn MessageWaiting(&mut self) -> ::windows::core::Result<i16>;
    fn SetDoNotDisturb(&mut self, fdonotdisturb: i16) -> ::windows::core::Result<()>;
    fn DoNotDisturb(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressVtbl {
        unsafe extern "system" fn State<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddressstate: *mut ADDRESS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *paddressstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressName<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProviderName<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TAPIObject<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TAPIObject() {
                ::core::result::Result::Ok(ok__) => {
                    *pptapiobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCall<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, laddresstype: i32, lmediatypes: i32, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCall(::core::mem::transmute_copy(&pdestaddress), ::core::mem::transmute_copy(&laddresstype), ::core::mem::transmute_copy(&lmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Calls() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCalls<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialableAddress<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdialableaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialableAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pdialableaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForwardInfoObject<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForwardInfoObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppforwardinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forward<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pforwardinfo: ::windows::core::RawPtr, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Forward(::core::mem::transmute(&pforwardinfo), ::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn CurrentForwardInfo<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentForwardInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppforwardinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageWaiting<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmessagewaiting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageWaiting(::core::mem::transmute_copy(&fmessagewaiting)).into()
        }
        unsafe extern "system" fn MessageWaiting<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmessagewaiting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageWaiting() {
                ::core::result::Result::Ok(ok__) => {
                    *pfmessagewaiting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotDisturb<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdonotdisturb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDoNotDisturb(::core::mem::transmute_copy(&fdonotdisturb)).into()
        }
        unsafe extern "system" fn DoNotDisturb<Impl: ITAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdonotdisturb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoNotDisturb() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdonotdisturb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            AddressName: AddressName::<Impl, IMPL_OFFSET>,
            ServiceProviderName: ServiceProviderName::<Impl, IMPL_OFFSET>,
            TAPIObject: TAPIObject::<Impl, IMPL_OFFSET>,
            CreateCall: CreateCall::<Impl, IMPL_OFFSET>,
            Calls: Calls::<Impl, IMPL_OFFSET>,
            EnumerateCalls: EnumerateCalls::<Impl, IMPL_OFFSET>,
            DialableAddress: DialableAddress::<Impl, IMPL_OFFSET>,
            CreateForwardInfoObject: CreateForwardInfoObject::<Impl, IMPL_OFFSET>,
            Forward: Forward::<Impl, IMPL_OFFSET>,
            CurrentForwardInfo: CurrentForwardInfo::<Impl, IMPL_OFFSET>,
            SetMessageWaiting: SetMessageWaiting::<Impl, IMPL_OFFSET>,
            MessageWaiting: MessageWaiting::<Impl, IMPL_OFFSET>,
            SetDoNotDisturb: SetDoNotDisturb::<Impl, IMPL_OFFSET>,
            DoNotDisturb: DoNotDisturb::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddress2Impl: Sized + IDispatchImpl + ITAddressImpl {
    fn Phones(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePhones(&mut self) -> ::windows::core::Result<IEnumPhone>;
    fn GetPhoneFromTerminal(&mut self, pterminal: ::core::option::Option<ITTerminal>) -> ::windows::core::Result<ITPhone>;
    fn PreferredPhones(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePreferredPhones(&mut self) -> ::windows::core::Result<IEnumPhone>;
    fn EventFilter(&mut self, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows::core::Result<i16>;
    fn SetEventFilter(&mut self, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::Result<()>;
    fn DeviceSpecific(&mut self, pcall: ::core::option::Option<ITCallInfo>, pparams: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn DeviceSpecificVariant(&mut self, pcall: ::core::option::Option<ITCallInfo>, vardevspecificbytearray: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NegotiateExtVersion(&mut self, llowversion: i32, lhighversion: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddress2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddress2Vtbl {
        unsafe extern "system" fn Phones<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phones() {
                ::core::result::Result::Ok(ok__) => {
                    *pphones = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePhones() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneFromTerminal<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhoneFromTerminal(::core::mem::transmute(&pterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPhones<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredPhones() {
                ::core::result::Result::Ok(ok__) => {
                    *pphones = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredPhones<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePreferredPhones() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventFilter<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *penable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn DeviceSpecific<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSpecific(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSpecificVariant(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Impl: ITAddress2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegotiateExtVersion(::core::mem::transmute_copy(&llowversion), ::core::mem::transmute_copy(&lhighversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *plextversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITAddressVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Phones: Phones::<Impl, IMPL_OFFSET>,
            EnumeratePhones: EnumeratePhones::<Impl, IMPL_OFFSET>,
            GetPhoneFromTerminal: GetPhoneFromTerminal::<Impl, IMPL_OFFSET>,
            PreferredPhones: PreferredPhones::<Impl, IMPL_OFFSET>,
            EnumeratePreferredPhones: EnumeratePreferredPhones::<Impl, IMPL_OFFSET>,
            EventFilter: EventFilter::<Impl, IMPL_OFFSET>,
            SetEventFilter: SetEventFilter::<Impl, IMPL_OFFSET>,
            DeviceSpecific: DeviceSpecific::<Impl, IMPL_OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Impl, IMPL_OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddress2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressCapabilitiesImpl: Sized + IDispatchImpl {
    fn AddressCapability(&mut self, addresscap: ADDRESS_CAPABILITY) -> ::windows::core::Result<i32>;
    fn AddressCapabilityString(&mut self, addresscapstring: ADDRESS_CAPABILITY_STRING) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CallTreatments(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCallTreatments(&mut self) -> ::windows::core::Result<IEnumBstr>;
    fn CompletionMessages(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCompletionMessages(&mut self) -> ::windows::core::Result<IEnumBstr>;
    fn DeviceClasses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDeviceClasses(&mut self) -> ::windows::core::Result<IEnumBstr>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressCapabilitiesVtbl {
        unsafe extern "system" fn AddressCapability<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressCapability(::core::mem::transmute_copy(&addresscap)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressCapabilityString<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressCapabilityString(::core::mem::transmute_copy(&addresscapstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilitystring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallTreatments<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallTreatments() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallTreatments<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcalltreatment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCallTreatments() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcalltreatment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompletionMessages<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompletionMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCompletionMessages<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcompletionmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCompletionMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcompletionmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceClasses<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDeviceClasses<Impl: ITAddressCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdeviceclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateDeviceClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdeviceclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddressCapability: AddressCapability::<Impl, IMPL_OFFSET>,
            AddressCapabilityString: AddressCapabilityString::<Impl, IMPL_OFFSET>,
            CallTreatments: CallTreatments::<Impl, IMPL_OFFSET>,
            EnumerateCallTreatments: EnumerateCallTreatments::<Impl, IMPL_OFFSET>,
            CompletionMessages: CompletionMessages::<Impl, IMPL_OFFSET>,
            EnumerateCompletionMessages: EnumerateCompletionMessages::<Impl, IMPL_OFFSET>,
            DeviceClasses: DeviceClasses::<Impl, IMPL_OFFSET>,
            EnumerateDeviceClasses: EnumerateDeviceClasses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressDeviceSpecificEventImpl: Sized + IDispatchImpl {
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn lParam1(&mut self) -> ::windows::core::Result<i32>;
    fn lParam2(&mut self) -> ::windows::core::Result<i32>;
    fn lParam3(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressDeviceSpecificEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressDeviceSpecificEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressDeviceSpecificEventVtbl {
        unsafe extern "system" fn Address<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam1() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam2() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam3() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam3 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Address: Address::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            lParam1: lParam1::<Impl, IMPL_OFFSET>,
            lParam2: lParam2::<Impl, IMPL_OFFSET>,
            lParam3: lParam3::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressDeviceSpecificEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressEventImpl: Sized + IDispatchImpl {
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn Event(&mut self) -> ::windows::core::Result<ADDRESS_EVENT>;
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressEventVtbl {
        unsafe extern "system" fn Address<Impl: ITAddressEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAddressEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ADDRESS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Impl: ITAddressEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Address: Address::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressTranslationImpl: Sized + IDispatchImpl {
    fn TranslateAddress(&mut self, paddresstotranslate: super::super::Foundation::BSTR, lcard: i32, ltranslateoptions: i32) -> ::windows::core::Result<ITAddressTranslationInfo>;
    fn TranslateDialog(&mut self, hwndowner: isize, paddressin: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EnumerateLocations(&mut self) -> ::windows::core::Result<IEnumLocation>;
    fn Locations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCallingCards(&mut self) -> ::windows::core::Result<IEnumCallingCard>;
    fn CallingCards(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressTranslationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressTranslationVtbl {
        unsafe extern "system" fn TranslateAddress<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresstotranslate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcard: i32, ltranslateoptions: i32, pptranslated: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateAddress(::core::mem::transmute_copy(&paddresstotranslate), ::core::mem::transmute_copy(&lcard), ::core::mem::transmute_copy(&ltranslateoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptranslated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateDialog<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: isize, paddressin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TranslateDialog(::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&paddressin)).into()
        }
        unsafe extern "system" fn EnumerateLocations<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumlocation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumlocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locations<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Locations() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallingCards<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcallingcard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCallingCards() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcallingcard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallingCards<Impl: ITAddressTranslationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallingCards() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TranslateAddress: TranslateAddress::<Impl, IMPL_OFFSET>,
            TranslateDialog: TranslateDialog::<Impl, IMPL_OFFSET>,
            EnumerateLocations: EnumerateLocations::<Impl, IMPL_OFFSET>,
            Locations: Locations::<Impl, IMPL_OFFSET>,
            EnumerateCallingCards: EnumerateCallingCards::<Impl, IMPL_OFFSET>,
            CallingCards: CallingCards::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressTranslation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressTranslationInfoImpl: Sized + IDispatchImpl {
    fn DialableString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayableString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentCountryCode(&mut self) -> ::windows::core::Result<i32>;
    fn DestinationCountryCode(&mut self) -> ::windows::core::Result<i32>;
    fn TranslationResults(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressTranslationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressTranslationInfoVtbl {
        unsafe extern "system" fn DialableString<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdialablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialableString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdialablestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayableString<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplayablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayableString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisplayablestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCountryCode<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *countrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationCountryCode<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationCountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *countrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslationResults<Impl: ITAddressTranslationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plresults: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslationResults() {
                ::core::result::Result::Ok(ok__) => {
                    *plresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DialableString: DialableString::<Impl, IMPL_OFFSET>,
            DisplayableString: DisplayableString::<Impl, IMPL_OFFSET>,
            CurrentCountryCode: CurrentCountryCode::<Impl, IMPL_OFFSET>,
            DestinationCountryCode: DestinationCountryCode::<Impl, IMPL_OFFSET>,
            TranslationResults: TranslationResults::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressTranslationInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentImpl: Sized + IDispatchImpl {
    fn EnumerateAgentSessions(&mut self) -> ::windows::core::Result<IEnumAgentSession>;
    fn CreateSession(&mut self, pacdgroup: ::core::option::Option<ITACDGroup>, paddress: ::core::option::Option<ITAddress>) -> ::windows::core::Result<ITAgentSession>;
    fn CreateSessionWithPIN(&mut self, pacdgroup: ::core::option::Option<ITACDGroup>, paddress: ::core::option::Option<ITAddress>, ppin: super::super::Foundation::BSTR) -> ::windows::core::Result<ITAgentSession>;
    fn ID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn User(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetState(&mut self, agentstate: AGENT_STATE) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<AGENT_STATE>;
    fn SetMeasurementPeriod(&mut self, lperiod: i32) -> ::windows::core::Result<()>;
    fn MeasurementPeriod(&mut self) -> ::windows::core::Result<i32>;
    fn OverallCallRate(&mut self) -> ::windows::core::Result<super::super::System::Com::CY>;
    fn NumberOfACDCalls(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfIncomingCalls(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfOutgoingCalls(&mut self) -> ::windows::core::Result<i32>;
    fn TotalACDTalkTime(&mut self) -> ::windows::core::Result<i32>;
    fn TotalACDCallTime(&mut self) -> ::windows::core::Result<i32>;
    fn TotalWrapUpTime(&mut self) -> ::windows::core::Result<i32>;
    fn AgentSessions(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentVtbl {
        unsafe extern "system" fn EnumerateAgentSessions<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAgentSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumagentsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSession(::core::mem::transmute(&pacdgroup), ::core::mem::transmute(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppagentsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionWithPIN<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSessionWithPIN(::core::mem::transmute(&pacdgroup), ::core::mem::transmute(&paddress), ::core::mem::transmute_copy(&ppin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppagentsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *ppuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agentstate: AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&agentstate)).into()
        }
        unsafe extern "system" fn State<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagentstate: *mut AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pagentstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeasurementPeriod<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMeasurementPeriod(::core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasurementPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *plperiod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallCallRate<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverallCallRate() {
                ::core::result::Result::Ok(ok__) => {
                    *pcycallrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfACDCalls<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfACDCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfIncomingCalls<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfIncomingCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfOutgoingCalls<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfOutgoingCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDTalkTime<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalACDTalkTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pltalktime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDCallTime<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalACDCallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalWrapUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwrapuptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentSessions<Impl: ITAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgentSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumerateAgentSessions: EnumerateAgentSessions::<Impl, IMPL_OFFSET>,
            CreateSession: CreateSession::<Impl, IMPL_OFFSET>,
            CreateSessionWithPIN: CreateSessionWithPIN::<Impl, IMPL_OFFSET>,
            ID: ID::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            SetMeasurementPeriod: SetMeasurementPeriod::<Impl, IMPL_OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Impl, IMPL_OFFSET>,
            OverallCallRate: OverallCallRate::<Impl, IMPL_OFFSET>,
            NumberOfACDCalls: NumberOfACDCalls::<Impl, IMPL_OFFSET>,
            NumberOfIncomingCalls: NumberOfIncomingCalls::<Impl, IMPL_OFFSET>,
            NumberOfOutgoingCalls: NumberOfOutgoingCalls::<Impl, IMPL_OFFSET>,
            TotalACDTalkTime: TotalACDTalkTime::<Impl, IMPL_OFFSET>,
            TotalACDCallTime: TotalACDCallTime::<Impl, IMPL_OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Impl, IMPL_OFFSET>,
            AgentSessions: AgentSessions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentEventImpl: Sized + IDispatchImpl {
    fn Agent(&mut self) -> ::windows::core::Result<ITAgent>;
    fn Event(&mut self) -> ::windows::core::Result<AGENT_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentEventVtbl {
        unsafe extern "system" fn Agent<Impl: ITAgentEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Agent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAgentEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Agent: Agent::<Impl, IMPL_OFFSET>, Event: Event::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentHandlerImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateAgent(&mut self) -> ::windows::core::Result<ITAgent>;
    fn CreateAgentWithID(&mut self, pid: super::super::Foundation::BSTR, ppin: super::super::Foundation::BSTR) -> ::windows::core::Result<ITAgent>;
    fn EnumerateACDGroups(&mut self) -> ::windows::core::Result<IEnumACDGroup>;
    fn EnumerateUsableAddresses(&mut self) -> ::windows::core::Result<IEnumAddress>;
    fn ACDGroups(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn UsableAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentHandlerVtbl {
        unsafe extern "system" fn Name<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgent<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAgent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgentWithID<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAgentWithID(::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&ppin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateACDGroups<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateACDGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumacdgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateUsableAddresses<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateUsableAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroups<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ACDGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsableAddresses<Impl: ITAgentHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsableAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            CreateAgent: CreateAgent::<Impl, IMPL_OFFSET>,
            CreateAgentWithID: CreateAgentWithID::<Impl, IMPL_OFFSET>,
            EnumerateACDGroups: EnumerateACDGroups::<Impl, IMPL_OFFSET>,
            EnumerateUsableAddresses: EnumerateUsableAddresses::<Impl, IMPL_OFFSET>,
            ACDGroups: ACDGroups::<Impl, IMPL_OFFSET>,
            UsableAddresses: UsableAddresses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentHandlerEventImpl: Sized + IDispatchImpl {
    fn AgentHandler(&mut self) -> ::windows::core::Result<ITAgentHandler>;
    fn Event(&mut self) -> ::windows::core::Result<AGENTHANDLER_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentHandlerEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandlerEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentHandlerEventVtbl {
        unsafe extern "system" fn AgentHandler<Impl: ITAgentHandlerEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagenthandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagenthandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAgentHandlerEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENTHANDLER_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AgentHandler: AgentHandler::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentHandlerEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentSessionImpl: Sized + IDispatchImpl {
    fn Agent(&mut self) -> ::windows::core::Result<ITAgent>;
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn ACDGroup(&mut self) -> ::windows::core::Result<ITACDGroup>;
    fn SetState(&mut self, sessionstate: AGENT_SESSION_STATE) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<AGENT_SESSION_STATE>;
    fn SessionStartTime(&mut self) -> ::windows::core::Result<f64>;
    fn SessionDuration(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfCalls(&mut self) -> ::windows::core::Result<i32>;
    fn TotalTalkTime(&mut self) -> ::windows::core::Result<i32>;
    fn AverageTalkTime(&mut self) -> ::windows::core::Result<i32>;
    fn TotalCallTime(&mut self) -> ::windows::core::Result<i32>;
    fn AverageCallTime(&mut self) -> ::windows::core::Result<i32>;
    fn TotalWrapUpTime(&mut self) -> ::windows::core::Result<i32>;
    fn AverageWrapUpTime(&mut self) -> ::windows::core::Result<i32>;
    fn ACDCallRate(&mut self) -> ::windows::core::Result<super::super::System::Com::CY>;
    fn LongestTimeToAnswer(&mut self) -> ::windows::core::Result<i32>;
    fn AverageTimeToAnswer(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentSessionVtbl {
        unsafe extern "system" fn Agent<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Agent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroup<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ACDGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *ppacdgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionstate: AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&sessionstate)).into()
        }
        unsafe extern "system" fn State<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: *mut AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *psessionstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesessionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatesessionstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionDuration<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *plduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfCalls<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTalkTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalTalkTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pltalktime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTalkTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AverageTalkTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pltalktime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageCallTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AverageCallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalWrapUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwrapuptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWrapUpTime<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AverageWrapUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwrapuptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDCallRate<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ACDCallRate() {
                ::core::result::Result::Ok(ok__) => {
                    *pcycallrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestTimeToAnswer<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongestTimeToAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    *planswertime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTimeToAnswer<Impl: ITAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AverageTimeToAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    *planswertime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Agent: Agent::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            ACDGroup: ACDGroup::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            SessionStartTime: SessionStartTime::<Impl, IMPL_OFFSET>,
            SessionDuration: SessionDuration::<Impl, IMPL_OFFSET>,
            NumberOfCalls: NumberOfCalls::<Impl, IMPL_OFFSET>,
            TotalTalkTime: TotalTalkTime::<Impl, IMPL_OFFSET>,
            AverageTalkTime: AverageTalkTime::<Impl, IMPL_OFFSET>,
            TotalCallTime: TotalCallTime::<Impl, IMPL_OFFSET>,
            AverageCallTime: AverageCallTime::<Impl, IMPL_OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Impl, IMPL_OFFSET>,
            AverageWrapUpTime: AverageWrapUpTime::<Impl, IMPL_OFFSET>,
            ACDCallRate: ACDCallRate::<Impl, IMPL_OFFSET>,
            LongestTimeToAnswer: LongestTimeToAnswer::<Impl, IMPL_OFFSET>,
            AverageTimeToAnswer: AverageTimeToAnswer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentSessionEventImpl: Sized + IDispatchImpl {
    fn Session(&mut self) -> ::windows::core::Result<ITAgentSession>;
    fn Event(&mut self) -> ::windows::core::Result<AGENT_SESSION_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentSessionEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSessionEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentSessionEventVtbl {
        unsafe extern "system" fn Session<Impl: ITAgentSessionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAgentSessionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_SESSION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Session: Session::<Impl, IMPL_OFFSET>, Event: Event::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentSessionEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait ITAllocatorPropertiesImpl: Sized {
    fn SetAllocatorProperties(&mut self, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::Result<()>;
    fn GetAllocatorProperties(&mut self) -> ::windows::core::Result<super::super::Media::DirectShow::ALLOCATOR_PROPERTIES>;
    fn SetAllocateBuffers(&mut self, ballocbuffers: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateBuffers(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBufferSize(&mut self, buffersize: u32) -> ::windows::core::Result<()>;
    fn GetBufferSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ITAllocatorPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAllocatorPropertiesVtbl {
        unsafe extern "system" fn SetAllocatorProperties<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllocatorProperties(::core::mem::transmute_copy(&pallocproperties)).into()
        }
        unsafe extern "system" fn GetAllocatorProperties<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllocatorProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pallocproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateBuffers<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballocbuffers: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllocateBuffers(::core::mem::transmute_copy(&ballocbuffers)).into()
        }
        unsafe extern "system" fn GetAllocateBuffers<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballocbuffers: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllocateBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *pballocbuffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferSize(::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn GetBufferSize<Impl: ITAllocatorPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pbuffersize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllocatorProperties: SetAllocatorProperties::<Impl, IMPL_OFFSET>,
            GetAllocatorProperties: GetAllocatorProperties::<Impl, IMPL_OFFSET>,
            SetAllocateBuffers: SetAllocateBuffers::<Impl, IMPL_OFFSET>,
            GetAllocateBuffers: GetAllocateBuffers::<Impl, IMPL_OFFSET>,
            SetBufferSize: SetBufferSize::<Impl, IMPL_OFFSET>,
            GetBufferSize: GetBufferSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAllocatorProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAutomatedPhoneControlImpl: Sized + IDispatchImpl {
    fn StartTone(&mut self, tone: PHONE_TONE, lduration: i32) -> ::windows::core::Result<()>;
    fn StopTone(&mut self) -> ::windows::core::Result<()>;
    fn Tone(&mut self) -> ::windows::core::Result<PHONE_TONE>;
    fn StartRinger(&mut self, lringmode: i32, lduration: i32) -> ::windows::core::Result<()>;
    fn StopRinger(&mut self) -> ::windows::core::Result<()>;
    fn Ringer(&mut self) -> ::windows::core::Result<i16>;
    fn SetPhoneHandlingEnabled(&mut self, fenabled: i16) -> ::windows::core::Result<()>;
    fn PhoneHandlingEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoEndOfNumberTimeout(&mut self, ltimeout: i32) -> ::windows::core::Result<()>;
    fn AutoEndOfNumberTimeout(&mut self) -> ::windows::core::Result<i32>;
    fn SetAutoDialtone(&mut self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoDialtone(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoStopTonesOnOnHook(&mut self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoStopTonesOnOnHook(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoStopRingOnOffHook(&mut self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoStopRingOnOffHook(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoKeypadTones(&mut self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoKeypadTones(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoKeypadTonesMinimumDuration(&mut self, lduration: i32) -> ::windows::core::Result<()>;
    fn AutoKeypadTonesMinimumDuration(&mut self) -> ::windows::core::Result<i32>;
    fn SetAutoVolumeControl(&mut self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoVolumeControl(&mut self) -> ::windows::core::Result<i16>;
    fn SetAutoVolumeControlStep(&mut self, lstepsize: i32) -> ::windows::core::Result<()>;
    fn AutoVolumeControlStep(&mut self) -> ::windows::core::Result<i32>;
    fn SetAutoVolumeControlRepeatDelay(&mut self, ldelay: i32) -> ::windows::core::Result<()>;
    fn AutoVolumeControlRepeatDelay(&mut self) -> ::windows::core::Result<i32>;
    fn SetAutoVolumeControlRepeatPeriod(&mut self, lperiod: i32) -> ::windows::core::Result<()>;
    fn AutoVolumeControlRepeatPeriod(&mut self) -> ::windows::core::Result<i32>;
    fn SelectCall(&mut self, pcall: ::core::option::Option<ITCallInfo>, fselectdefaultterminals: i16) -> ::windows::core::Result<()>;
    fn UnselectCall(&mut self, pcall: ::core::option::Option<ITCallInfo>) -> ::windows::core::Result<()>;
    fn EnumerateSelectedCalls(&mut self) -> ::windows::core::Result<IEnumCall>;
    fn SelectedCalls(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAutomatedPhoneControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAutomatedPhoneControlVtbl {
        unsafe extern "system" fn StartTone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tone: PHONE_TONE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTone(::core::mem::transmute_copy(&tone), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopTone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopTone().into()
        }
        unsafe extern "system" fn Tone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptone: *mut PHONE_TONE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tone() {
                ::core::result::Result::Ok(ok__) => {
                    *ptone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRinger<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartRinger(::core::mem::transmute_copy(&lringmode), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopRinger<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopRinger().into()
        }
        unsafe extern "system" fn Ringer<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfringing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ringer() {
                ::core::result::Result::Ok(ok__) => {
                    *pfringing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhoneHandlingEnabled<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhoneHandlingEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn PhoneHandlingEnabled<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneHandlingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoEndOfNumberTimeout<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoEndOfNumberTimeout(::core::mem::transmute_copy(&ltimeout)).into()
        }
        unsafe extern "system" fn AutoEndOfNumberTimeout<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltimeout: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoEndOfNumberTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *pltimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDialtone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoDialtone(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoDialtone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoDialtone() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopTonesOnOnHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoStopTonesOnOnHook(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopTonesOnOnHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoStopTonesOnOnHook() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopRingOnOffHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoStopRingOnOffHook(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopRingOnOffHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoStopRingOnOffHook() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTones<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoKeypadTones(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoKeypadTones<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoKeypadTones() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTonesMinimumDuration<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoKeypadTonesMinimumDuration(::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn AutoKeypadTonesMinimumDuration<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoKeypadTonesMinimumDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *plduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControl<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoVolumeControl(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoVolumeControl<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControl() {
                ::core::result::Result::Ok(ok__) => {
                    *fenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlStep<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstepsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoVolumeControlStep(::core::mem::transmute_copy(&lstepsize)).into()
        }
        unsafe extern "system" fn AutoVolumeControlStep<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstepsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControlStep() {
                ::core::result::Result::Ok(ok__) => {
                    *plstepsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatDelay<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoVolumeControlRepeatDelay(::core::mem::transmute_copy(&ldelay)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatDelay<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControlRepeatDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatPeriod<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoVolumeControlRepeatPeriod(::core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatPeriod<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControlRepeatPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *plperiod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectCall<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fselectdefaultterminals: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectCall(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&fselectdefaultterminals)).into()
        }
        unsafe extern "system" fn UnselectCall<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnselectCall(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn EnumerateSelectedCalls<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSelectedCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedCalls<Impl: ITAutomatedPhoneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartTone: StartTone::<Impl, IMPL_OFFSET>,
            StopTone: StopTone::<Impl, IMPL_OFFSET>,
            Tone: Tone::<Impl, IMPL_OFFSET>,
            StartRinger: StartRinger::<Impl, IMPL_OFFSET>,
            StopRinger: StopRinger::<Impl, IMPL_OFFSET>,
            Ringer: Ringer::<Impl, IMPL_OFFSET>,
            SetPhoneHandlingEnabled: SetPhoneHandlingEnabled::<Impl, IMPL_OFFSET>,
            PhoneHandlingEnabled: PhoneHandlingEnabled::<Impl, IMPL_OFFSET>,
            SetAutoEndOfNumberTimeout: SetAutoEndOfNumberTimeout::<Impl, IMPL_OFFSET>,
            AutoEndOfNumberTimeout: AutoEndOfNumberTimeout::<Impl, IMPL_OFFSET>,
            SetAutoDialtone: SetAutoDialtone::<Impl, IMPL_OFFSET>,
            AutoDialtone: AutoDialtone::<Impl, IMPL_OFFSET>,
            SetAutoStopTonesOnOnHook: SetAutoStopTonesOnOnHook::<Impl, IMPL_OFFSET>,
            AutoStopTonesOnOnHook: AutoStopTonesOnOnHook::<Impl, IMPL_OFFSET>,
            SetAutoStopRingOnOffHook: SetAutoStopRingOnOffHook::<Impl, IMPL_OFFSET>,
            AutoStopRingOnOffHook: AutoStopRingOnOffHook::<Impl, IMPL_OFFSET>,
            SetAutoKeypadTones: SetAutoKeypadTones::<Impl, IMPL_OFFSET>,
            AutoKeypadTones: AutoKeypadTones::<Impl, IMPL_OFFSET>,
            SetAutoKeypadTonesMinimumDuration: SetAutoKeypadTonesMinimumDuration::<Impl, IMPL_OFFSET>,
            AutoKeypadTonesMinimumDuration: AutoKeypadTonesMinimumDuration::<Impl, IMPL_OFFSET>,
            SetAutoVolumeControl: SetAutoVolumeControl::<Impl, IMPL_OFFSET>,
            AutoVolumeControl: AutoVolumeControl::<Impl, IMPL_OFFSET>,
            SetAutoVolumeControlStep: SetAutoVolumeControlStep::<Impl, IMPL_OFFSET>,
            AutoVolumeControlStep: AutoVolumeControlStep::<Impl, IMPL_OFFSET>,
            SetAutoVolumeControlRepeatDelay: SetAutoVolumeControlRepeatDelay::<Impl, IMPL_OFFSET>,
            AutoVolumeControlRepeatDelay: AutoVolumeControlRepeatDelay::<Impl, IMPL_OFFSET>,
            SetAutoVolumeControlRepeatPeriod: SetAutoVolumeControlRepeatPeriod::<Impl, IMPL_OFFSET>,
            AutoVolumeControlRepeatPeriod: AutoVolumeControlRepeatPeriod::<Impl, IMPL_OFFSET>,
            SelectCall: SelectCall::<Impl, IMPL_OFFSET>,
            UnselectCall: UnselectCall::<Impl, IMPL_OFFSET>,
            EnumerateSelectedCalls: EnumerateSelectedCalls::<Impl, IMPL_OFFSET>,
            SelectedCalls: SelectedCalls::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAutomatedPhoneControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicAudioTerminalImpl: Sized + IDispatchImpl {
    fn SetVolume(&mut self, lvolume: i32) -> ::windows::core::Result<()>;
    fn Volume(&mut self) -> ::windows::core::Result<i32>;
    fn SetBalance(&mut self, lbalance: i32) -> ::windows::core::Result<()>;
    fn Balance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicAudioTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicAudioTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITBasicAudioTerminalVtbl {
        unsafe extern "system" fn SetVolume<Impl: ITBasicAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn Volume<Impl: ITBasicAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBalance<Impl: ITBasicAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBalance(::core::mem::transmute_copy(&lbalance)).into()
        }
        unsafe extern "system" fn Balance<Impl: ITBasicAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Balance() {
                ::core::result::Result::Ok(ok__) => {
                    *plbalance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            SetBalance: SetBalance::<Impl, IMPL_OFFSET>,
            Balance: Balance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicAudioTerminal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicCallControlImpl: Sized + IDispatchImpl {
    fn Connect(&mut self, fsync: i16) -> ::windows::core::Result<()>;
    fn Answer(&mut self) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self, code: DISCONNECT_CODE) -> ::windows::core::Result<()>;
    fn Hold(&mut self, fhold: i16) -> ::windows::core::Result<()>;
    fn HandoffDirect(&mut self, papplicationname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HandoffIndirect(&mut self, lmediatype: i32) -> ::windows::core::Result<()>;
    fn Conference(&mut self, pcall: ::core::option::Option<ITBasicCallControl>, fsync: i16) -> ::windows::core::Result<()>;
    fn Transfer(&mut self, pcall: ::core::option::Option<ITBasicCallControl>, fsync: i16) -> ::windows::core::Result<()>;
    fn BlindTransfer(&mut self, pdestaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SwapHold(&mut self, pcall: ::core::option::Option<ITBasicCallControl>) -> ::windows::core::Result<()>;
    fn ParkDirect(&mut self, pparkaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ParkIndirect(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Unpark(&mut self) -> ::windows::core::Result<()>;
    fn SetQOS(&mut self, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::Result<()>;
    fn Pickup(&mut self, pgroupid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Dial(&mut self, pdestaddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Finish(&mut self, finishmode: FINISH_MODE) -> ::windows::core::Result<()>;
    fn RemoveFromConference(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicCallControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITBasicCallControlVtbl {
        unsafe extern "system" fn Connect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Answer<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Answer().into()
        }
        unsafe extern "system" fn Disconnect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: DISCONNECT_CODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(::core::mem::transmute_copy(&code)).into()
        }
        unsafe extern "system" fn Hold<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fhold: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hold(::core::mem::transmute_copy(&fhold)).into()
        }
        unsafe extern "system" fn HandoffDirect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandoffDirect(::core::mem::transmute_copy(&papplicationname)).into()
        }
        unsafe extern "system" fn HandoffIndirect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandoffIndirect(::core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn Conference<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Conference(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Transfer<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Transfer(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn BlindTransfer<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BlindTransfer(::core::mem::transmute_copy(&pdestaddress)).into()
        }
        unsafe extern "system" fn SwapHold<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwapHold(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn ParkDirect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparkaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParkDirect(::core::mem::transmute_copy(&pparkaddress)).into()
        }
        unsafe extern "system" fn ParkIndirect<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnondiraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParkIndirect() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnondiraddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unpark<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unpark().into()
        }
        unsafe extern "system" fn SetQOS<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQOS(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&servicelevel)).into()
        }
        unsafe extern "system" fn Pickup<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroupid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pickup(::core::mem::transmute_copy(&pgroupid)).into()
        }
        unsafe extern "system" fn Dial<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dial(::core::mem::transmute_copy(&pdestaddress)).into()
        }
        unsafe extern "system" fn Finish<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finishmode: FINISH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&finishmode)).into()
        }
        unsafe extern "system" fn RemoveFromConference<Impl: ITBasicCallControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromConference().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Answer: Answer::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            Hold: Hold::<Impl, IMPL_OFFSET>,
            HandoffDirect: HandoffDirect::<Impl, IMPL_OFFSET>,
            HandoffIndirect: HandoffIndirect::<Impl, IMPL_OFFSET>,
            Conference: Conference::<Impl, IMPL_OFFSET>,
            Transfer: Transfer::<Impl, IMPL_OFFSET>,
            BlindTransfer: BlindTransfer::<Impl, IMPL_OFFSET>,
            SwapHold: SwapHold::<Impl, IMPL_OFFSET>,
            ParkDirect: ParkDirect::<Impl, IMPL_OFFSET>,
            ParkIndirect: ParkIndirect::<Impl, IMPL_OFFSET>,
            Unpark: Unpark::<Impl, IMPL_OFFSET>,
            SetQOS: SetQOS::<Impl, IMPL_OFFSET>,
            Pickup: Pickup::<Impl, IMPL_OFFSET>,
            Dial: Dial::<Impl, IMPL_OFFSET>,
            Finish: Finish::<Impl, IMPL_OFFSET>,
            RemoveFromConference: RemoveFromConference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicCallControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicCallControl2Impl: Sized + IDispatchImpl + ITBasicCallControlImpl {
    fn RequestTerminal(&mut self, bstrterminalclassguid: super::super::Foundation::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
    fn SelectTerminalOnCall(&mut self, pterminal: ::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn UnselectTerminalOnCall(&mut self, pterminal: ::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicCallControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITBasicCallControl2Vtbl {
        unsafe extern "system" fn RequestTerminal<Impl: ITBasicCallControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrterminalclassguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestTerminal(::core::mem::transmute_copy(&bstrterminalclassguid), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectTerminalOnCall<Impl: ITBasicCallControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectTerminalOnCall(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminalOnCall<Impl: ITBasicCallControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnselectTerminalOnCall(::core::mem::transmute(&pterminal)).into()
        }
        Self {
            base: ITBasicCallControlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RequestTerminal: RequestTerminal::<Impl, IMPL_OFFSET>,
            SelectTerminalOnCall: SelectTerminalOnCall::<Impl, IMPL_OFFSET>,
            UnselectTerminalOnCall: UnselectTerminalOnCall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicCallControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallHubImpl: Sized + IDispatchImpl {
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn EnumerateCalls(&mut self) -> ::windows::core::Result<IEnumCall>;
    fn Calls(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NumCalls(&mut self) -> ::windows::core::Result<i32>;
    fn State(&mut self) -> ::windows::core::Result<CALLHUB_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallHubVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHubImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallHubVtbl {
        unsafe extern "system" fn Clear<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn EnumerateCalls<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcalls: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Calls() {
                ::core::result::Result::Ok(ok__) => {
                    *pcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumCalls<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITCallHubImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut CALLHUB_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Clear: Clear::<Impl, IMPL_OFFSET>,
            EnumerateCalls: EnumerateCalls::<Impl, IMPL_OFFSET>,
            Calls: Calls::<Impl, IMPL_OFFSET>,
            NumCalls: NumCalls::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallHub as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallHubEventImpl: Sized + IDispatchImpl {
    fn Event(&mut self) -> ::windows::core::Result<CALLHUB_EVENT>;
    fn CallHub(&mut self) -> ::windows::core::Result<ITCallHub>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallHubEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHubEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallHubEventVtbl {
        unsafe extern "system" fn Event<Impl: ITCallHubEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut CALLHUB_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Impl: ITCallHubEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHub() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITCallHubEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Event: Event::<Impl, IMPL_OFFSET>,
            CallHub: CallHub::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallHubEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfoImpl: Sized + IDispatchImpl {
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn CallState(&mut self) -> ::windows::core::Result<CALL_STATE>;
    fn Privilege(&mut self) -> ::windows::core::Result<CALL_PRIVILEGE>;
    fn CallHub(&mut self) -> ::windows::core::Result<ITCallHub>;
    fn CallInfoLong(&mut self, callinfolong: CALLINFO_LONG) -> ::windows::core::Result<i32>;
    fn SetCallInfoLong(&mut self, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::Result<()>;
    fn CallInfoString(&mut self, callinfostring: CALLINFO_STRING) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCallInfoString(&mut self, callinfostring: CALLINFO_STRING, pcallinfostring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CallInfoBuffer(&mut self, callinfobuffer: CALLINFO_BUFFER) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetCallInfoBuffer(&mut self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetCallInfoBuffer(&mut self, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetCallInfoBuffer(&mut self, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows::core::Result<()>;
    fn ReleaseUserUserInfo(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallInfoVtbl {
        unsafe extern "system" fn Address<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallState<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallState() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivilege: *mut CALL_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Privilege() {
                ::core::result::Result::Ok(ok__) => {
                    *pprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHub() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallInfoLong<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallInfoLong(::core::mem::transmute_copy(&callinfolong)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcallinfolongval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoLong<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallInfoLong(::core::mem::transmute_copy(&callinfolong), ::core::mem::transmute_copy(&lcallinfolongval)).into()
        }
        unsafe extern "system" fn CallInfoString<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallInfoString(::core::mem::transmute_copy(&callinfostring)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfostring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoString<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, pcallinfostring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallInfoString(::core::mem::transmute_copy(&callinfostring), ::core::mem::transmute_copy(&pcallinfostring)).into()
        }
        unsafe extern "system" fn CallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfobuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn GetCallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppcallinfobuffer)).into()
        }
        unsafe extern "system" fn SetCallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn ReleaseUserUserInfo<Impl: ITCallInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseUserUserInfo().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Address: Address::<Impl, IMPL_OFFSET>,
            CallState: CallState::<Impl, IMPL_OFFSET>,
            Privilege: Privilege::<Impl, IMPL_OFFSET>,
            CallHub: CallHub::<Impl, IMPL_OFFSET>,
            CallInfoLong: CallInfoLong::<Impl, IMPL_OFFSET>,
            SetCallInfoLong: SetCallInfoLong::<Impl, IMPL_OFFSET>,
            CallInfoString: CallInfoString::<Impl, IMPL_OFFSET>,
            SetCallInfoString: SetCallInfoString::<Impl, IMPL_OFFSET>,
            CallInfoBuffer: CallInfoBuffer::<Impl, IMPL_OFFSET>,
            SetCallInfoBuffer: SetCallInfoBuffer::<Impl, IMPL_OFFSET>,
            GetCallInfoBuffer: GetCallInfoBuffer::<Impl, IMPL_OFFSET>,
            SetCallInfoBuffer: SetCallInfoBuffer::<Impl, IMPL_OFFSET>,
            ReleaseUserUserInfo: ReleaseUserUserInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfo2Impl: Sized + IDispatchImpl + ITCallInfoImpl {
    fn EventFilter(&mut self, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows::core::Result<i16>;
    fn SetEventFilter(&mut self, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallInfo2Vtbl {
        unsafe extern "system" fn EventFilter<Impl: ITCallInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *penable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITCallInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent), ::core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base: ITCallInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventFilter: EventFilter::<Impl, IMPL_OFFSET>,
            SetEventFilter: SetEventFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfoChangeEventImpl: Sized + IDispatchImpl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Cause(&mut self) -> ::windows::core::Result<CALLINFOCHANGE_CAUSE>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfoChangeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfoChangeEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallInfoChangeEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallInfoChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITCallInfoChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcic: *mut CALLINFOCHANGE_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallInfoChangeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Cause: Cause::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfoChangeEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallMediaEventImpl: Sized + IDispatchImpl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Event(&mut self) -> ::windows::core::Result<CALL_MEDIA_EVENT>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Stream(&mut self) -> ::windows::core::Result<ITStream>;
    fn Cause(&mut self) -> ::windows::core::Result<CALL_MEDIA_EVENT_CAUSE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallMediaEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallMediaEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallmediaevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITCallMediaEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcause = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Stream: Stream::<Impl, IMPL_OFFSET>,
            Cause: Cause::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallMediaEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallNotificationEventImpl: Sized + IDispatchImpl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Event(&mut self) -> ::windows::core::Result<CALL_NOTIFICATION_EVENT>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallNotificationEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallNotificationEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallNotificationEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallNotificationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITCallNotificationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallnotificationevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallNotificationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallNotificationEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallStateEventImpl: Sized + IDispatchImpl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn State(&mut self) -> ::windows::core::Result<CALL_STATE>;
    fn Cause(&mut self) -> ::windows::core::Result<CALL_STATE_EVENT_CAUSE>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallStateEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallStateEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallStateEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallStateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITCallStateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITCallStateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcec: *mut CALL_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallStateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Cause: Cause::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallStateEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallingCardImpl: Sized + IDispatchImpl {
    fn PermanentCardID(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfDigits(&mut self) -> ::windows::core::Result<i32>;
    fn Options(&mut self) -> ::windows::core::Result<i32>;
    fn CardName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SameAreaDialingRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LongDistanceDialingRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InternationalDialingRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallingCardVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCardImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallingCardVtbl {
        unsafe extern "system" fn PermanentCardID<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermanentCardID() {
                ::core::result::Result::Ok(ok__) => {
                    *plcardid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfDigits<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldigits: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *pldigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *ploptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardName<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcardname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcardname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SameAreaDialingRule<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SameAreaDialingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pprule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceDialingRule<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongDistanceDialingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pprule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternationalDialingRule<Impl: ITCallingCardImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InternationalDialingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pprule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PermanentCardID: PermanentCardID::<Impl, IMPL_OFFSET>,
            NumberOfDigits: NumberOfDigits::<Impl, IMPL_OFFSET>,
            Options: Options::<Impl, IMPL_OFFSET>,
            CardName: CardName::<Impl, IMPL_OFFSET>,
            SameAreaDialingRule: SameAreaDialingRule::<Impl, IMPL_OFFSET>,
            LongDistanceDialingRule: LongDistanceDialingRule::<Impl, IMPL_OFFSET>,
            InternationalDialingRule: InternationalDialingRule::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallingCard as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCollectionImpl: Sized + IDispatchImpl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ITCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *lcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ITCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ITCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCollection2Impl: Sized + IDispatchImpl + ITCollectionImpl {
    fn Add(&mut self, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCollection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCollection2Vtbl {
        unsafe extern "system" fn Add<Impl: ITCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pvariant)).into()
        }
        unsafe extern "system" fn Remove<Impl: ITCollection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        Self { base: ITCollectionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Add: Add::<Impl, IMPL_OFFSET>, Remove: Remove::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCollection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCustomToneImpl: Sized + IDispatchImpl {
    fn Frequency(&mut self) -> ::windows::core::Result<i32>;
    fn SetFrequency(&mut self, lfrequency: i32) -> ::windows::core::Result<()>;
    fn CadenceOn(&mut self) -> ::windows::core::Result<i32>;
    fn SetCadenceOn(&mut self, cadenceon: i32) -> ::windows::core::Result<()>;
    fn CadenceOff(&mut self) -> ::windows::core::Result<i32>;
    fn SetCadenceOff(&mut self, lcadenceoff: i32) -> ::windows::core::Result<()>;
    fn Volume(&mut self) -> ::windows::core::Result<i32>;
    fn SetVolume(&mut self, lvolume: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCustomToneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomToneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCustomToneVtbl {
        unsafe extern "system" fn Frequency<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frequency() {
                ::core::result::Result::Ok(ok__) => {
                    *plfrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfrequency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrequency(::core::mem::transmute_copy(&lfrequency)).into()
        }
        unsafe extern "system" fn CadenceOn<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcadenceon: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CadenceOn() {
                ::core::result::Result::Ok(ok__) => {
                    *plcadenceon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOn<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cadenceon: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCadenceOn(::core::mem::transmute_copy(&cadenceon)).into()
        }
        unsafe extern "system" fn CadenceOff<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcadenceoff: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CadenceOff() {
                ::core::result::Result::Ok(ok__) => {
                    *plcadenceoff = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOff<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcadenceoff: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCadenceOff(::core::mem::transmute_copy(&lcadenceoff)).into()
        }
        unsafe extern "system" fn Volume<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: ITCustomToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Frequency: Frequency::<Impl, IMPL_OFFSET>,
            SetFrequency: SetFrequency::<Impl, IMPL_OFFSET>,
            CadenceOn: CadenceOn::<Impl, IMPL_OFFSET>,
            SetCadenceOn: SetCadenceOn::<Impl, IMPL_OFFSET>,
            CadenceOff: CadenceOff::<Impl, IMPL_OFFSET>,
            SetCadenceOff: SetCadenceOff::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCustomTone as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDetectToneImpl: Sized + IDispatchImpl {
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<i32>;
    fn SetDuration(&mut self, lduration: i32) -> ::windows::core::Result<()>;
    fn Frequency(&mut self, index: i32) -> ::windows::core::Result<i32>;
    fn SetFrequency(&mut self, index: i32, lfrequency: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDetectToneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectToneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDetectToneVtbl {
        unsafe extern "system" fn AppSpecific<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn Duration<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *plduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn Frequency<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frequency(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *plfrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Impl: ITDetectToneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, lfrequency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrequency(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&lfrequency)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AppSpecific: AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific: SetAppSpecific::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Frequency: Frequency::<Impl, IMPL_OFFSET>,
            SetFrequency: SetFrequency::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDetectTone as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitDetectionEventImpl: Sized + IDispatchImpl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Digit(&mut self) -> ::windows::core::Result<u8>;
    fn DigitMode(&mut self) -> ::windows::core::Result<i32>;
    fn TickCount(&mut self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitDetectionEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitDetectionEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDigitDetectionEventVtbl {
        unsafe extern "system" fn Call<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digit<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucdigit: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Digit() {
                ::core::result::Result::Ok(ok__) => {
                    *pucdigit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitMode<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigitmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DigitMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pdigitmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Digit: Digit::<Impl, IMPL_OFFSET>,
            DigitMode: DigitMode::<Impl, IMPL_OFFSET>,
            TickCount: TickCount::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitDetectionEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitGenerationEventImpl: Sized + IDispatchImpl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn GenerationTermination(&mut self) -> ::windows::core::Result<i32>;
    fn TickCount(&mut self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitGenerationEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitGenerationEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDigitGenerationEventVtbl {
        unsafe extern "system" fn Call<Impl: ITDigitGenerationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerationTermination<Impl: ITDigitGenerationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plgenerationtermination: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerationTermination() {
                ::core::result::Result::Ok(ok__) => {
                    *plgenerationtermination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitGenerationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitGenerationEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            GenerationTermination: GenerationTermination::<Impl, IMPL_OFFSET>,
            TickCount: TickCount::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitGenerationEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitsGatheredEventImpl: Sized + IDispatchImpl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Digits(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GatherTermination(&mut self) -> ::windows::core::Result<TAPI_GATHERTERM>;
    fn TickCount(&mut self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitsGatheredEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitsGatheredEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDigitsGatheredEventVtbl {
        unsafe extern "system" fn Call<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digits<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdigits: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Digits() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GatherTermination<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgathertermination: *mut TAPI_GATHERTERM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GatherTermination() {
                ::core::result::Result::Ok(ok__) => {
                    *pgathertermination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitsGatheredEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Digits: Digits::<Impl, IMPL_OFFSET>,
            GatherTermination: GatherTermination::<Impl, IMPL_OFFSET>,
            TickCount: TickCount::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitsGatheredEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryImpl: Sized + IDispatchImpl {
    fn DirectoryType(&mut self) -> ::windows::core::Result<DIRECTORY_TYPE>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsDynamic(&mut self) -> ::windows::core::Result<i16>;
    fn DefaultObjectTTL(&mut self) -> ::windows::core::Result<i32>;
    fn SetDefaultObjectTTL(&mut self, ttl: i32) -> ::windows::core::Result<()>;
    fn EnableAutoRefresh(&mut self, fenable: i16) -> ::windows::core::Result<()>;
    fn Connect(&mut self, fsecure: i16) -> ::windows::core::Result<()>;
    fn Bind(&mut self, pdomainname: super::super::Foundation::BSTR, pusername: super::super::Foundation::BSTR, ppassword: super::super::Foundation::BSTR, lflags: i32) -> ::windows::core::Result<()>;
    fn AddDirectoryObject(&mut self, pdirectoryobject: ::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn ModifyDirectoryObject(&mut self, pdirectoryobject: ::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn RefreshDirectoryObject(&mut self, pdirectoryobject: ::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn DeleteDirectoryObject(&mut self, pdirectoryobject: ::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn DirectoryObjects(&mut self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDirectoryObjects(&mut self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: super::super::Foundation::BSTR) -> ::windows::core::Result<IEnumDirectoryObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryVtbl {
        unsafe extern "system" fn DirectoryType<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectorytype: *mut DIRECTORY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirectorytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDynamic<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdynamic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDynamic() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdynamic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultObjectTTL<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultObjectTTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultObjectTTL<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ttl: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultObjectTTL(::core::mem::transmute_copy(&ttl)).into()
        }
        unsafe extern "system" fn EnableAutoRefresh<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableAutoRefresh(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn Connect<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsecure: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&fsecure)).into()
        }
        unsafe extern "system" fn Bind<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Bind(::core::mem::transmute_copy(&pdomainname), ::core::mem::transmute_copy(&pusername), ::core::mem::transmute_copy(&ppassword), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn ModifyDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifyDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn RefreshDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn DeleteDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn DirectoryObjects<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryObjects(::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDirectoryObjects<Impl: ITDirectoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenumobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateDirectoryObjects(::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DirectoryType: DirectoryType::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            IsDynamic: IsDynamic::<Impl, IMPL_OFFSET>,
            DefaultObjectTTL: DefaultObjectTTL::<Impl, IMPL_OFFSET>,
            SetDefaultObjectTTL: SetDefaultObjectTTL::<Impl, IMPL_OFFSET>,
            EnableAutoRefresh: EnableAutoRefresh::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Bind: Bind::<Impl, IMPL_OFFSET>,
            AddDirectoryObject: AddDirectoryObject::<Impl, IMPL_OFFSET>,
            ModifyDirectoryObject: ModifyDirectoryObject::<Impl, IMPL_OFFSET>,
            RefreshDirectoryObject: RefreshDirectoryObject::<Impl, IMPL_OFFSET>,
            DeleteDirectoryObject: DeleteDirectoryObject::<Impl, IMPL_OFFSET>,
            DirectoryObjects: DirectoryObjects::<Impl, IMPL_OFFSET>,
            EnumerateDirectoryObjects: EnumerateDirectoryObjects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectImpl: Sized + IDispatchImpl {
    fn ObjectType(&mut self) -> ::windows::core::Result<DIRECTORY_OBJECT_TYPE>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, pname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DialableAddrs(&mut self, dwaddresstype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDialableAddrs(&mut self, dwaddresstype: u32) -> ::windows::core::Result<IEnumDialableAddrs>;
    fn SecurityDescriptor(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(&mut self, psecdes: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryObjectVtbl {
        unsafe extern "system" fn ObjectType<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    *pobjecttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn DialableAddrs<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddresstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialableAddrs(::core::mem::transmute_copy(&dwaddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDialableAddrs<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddresstype: u32, ppenumdialableaddrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateDialableAddrs(::core::mem::transmute_copy(&dwaddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdialableaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecdes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecdes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: ITDirectoryObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecdes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute(&psecdes)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ObjectType: ObjectType::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            DialableAddrs: DialableAddrs::<Impl, IMPL_OFFSET>,
            EnumerateDialableAddrs: EnumerateDialableAddrs::<Impl, IMPL_OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Impl, IMPL_OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectConferenceImpl: Sized + IDispatchImpl {
    fn Protocol(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Originator(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOriginator(&mut self, poriginator: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AdvertisingScope(&mut self) -> ::windows::core::Result<RND_ADVERTISING_SCOPE>;
    fn SetAdvertisingScope(&mut self, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::Result<()>;
    fn Url(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetUrl(&mut self, purl: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, pdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsEncrypted(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsEncrypted(&mut self, fencrypted: i16) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStartTime(&mut self, date: f64) -> ::windows::core::Result<()>;
    fn StopTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStopTime(&mut self, date: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectConferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryObjectConferenceVtbl {
        unsafe extern "system" fn Protocol<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Originator<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginator: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Originator() {
                ::core::result::Result::Ok(ok__) => {
                    *pporiginator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginator<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poriginator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOriginator(::core::mem::transmute_copy(&poriginator)).into()
        }
        unsafe extern "system" fn AdvertisingScope<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisingScope() {
                ::core::result::Result::Ok(ok__) => {
                    *padvertisingscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisingScope<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvertisingScope(::core::mem::transmute_copy(&advertisingscope)).into()
        }
        unsafe extern "system" fn Url<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *ppurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUrl(::core::mem::transmute_copy(&purl)).into()
        }
        unsafe extern "system" fn Description<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn IsEncrypted<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfencrypted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEncrypted() {
                ::core::result::Result::Ok(ok__) => {
                    *pfencrypted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEncrypted<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fencrypted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEncrypted(::core::mem::transmute_copy(&fencrypted)).into()
        }
        unsafe extern "system" fn StartTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn StopTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopTime(::core::mem::transmute_copy(&date)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            Originator: Originator::<Impl, IMPL_OFFSET>,
            SetOriginator: SetOriginator::<Impl, IMPL_OFFSET>,
            AdvertisingScope: AdvertisingScope::<Impl, IMPL_OFFSET>,
            SetAdvertisingScope: SetAdvertisingScope::<Impl, IMPL_OFFSET>,
            Url: Url::<Impl, IMPL_OFFSET>,
            SetUrl: SetUrl::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            IsEncrypted: IsEncrypted::<Impl, IMPL_OFFSET>,
            SetIsEncrypted: SetIsEncrypted::<Impl, IMPL_OFFSET>,
            StartTime: StartTime::<Impl, IMPL_OFFSET>,
            SetStartTime: SetStartTime::<Impl, IMPL_OFFSET>,
            StopTime: StopTime::<Impl, IMPL_OFFSET>,
            SetStopTime: SetStopTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObjectConference as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectUserImpl: Sized + IDispatchImpl {
    fn IPPhonePrimary(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetIPPhonePrimary(&mut self, pname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryObjectUserVtbl {
        unsafe extern "system" fn IPPhonePrimary<Impl: ITDirectoryObjectUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IPPhonePrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPPhonePrimary<Impl: ITDirectoryObjectUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIPPhonePrimary(::core::mem::transmute_copy(&pname)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IPPhonePrimary: IPPhonePrimary::<Impl, IMPL_OFFSET>,
            SetIPPhonePrimary: SetIPPhonePrimary::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObjectUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDispatchMapperImpl: Sized + IDispatchImpl {
    fn QueryDispatchInterface(&mut self, piid: super::super::Foundation::BSTR, pinterfacetomap: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDispatchMapperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDispatchMapperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDispatchMapperVtbl {
        unsafe extern "system" fn QueryDispatchInterface<Impl: ITDispatchMapperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pinterfacetomap: ::windows::core::RawPtr, ppreturnedinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDispatchInterface(::core::mem::transmute_copy(&piid), ::core::mem::transmute(&pinterfacetomap)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreturnedinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), QueryDispatchInterface: QueryDispatchInterface::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDispatchMapper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITFileTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Track(&mut self) -> ::windows::core::Result<ITFileTrack>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn State(&mut self) -> ::windows::core::Result<TERMINAL_MEDIA_STATE>;
    fn Cause(&mut self) -> ::windows::core::Result<FT_STATE_EVENT_CAUSE>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITFileTerminalEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITFileTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrackterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track() {
                ::core::result::Result::Ok(ok__) => {
                    *pptrackterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcause: *mut FT_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcause = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITFileTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Track: Track::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Cause: Cause::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITFileTerminalEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITFileTrackImpl: Sized + IDispatchImpl {
    fn Format(&mut self) -> ::windows::core::Result<*mut super::super::Media::DirectShow::AM_MEDIA_TYPE>;
    fn SetFormat(&mut self, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
    fn ControllingTerminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn AudioFormatForScripting(&mut self) -> ::windows::core::Result<ITScriptableAudioFormat>;
    fn SetAudioFormatForScripting(&mut self, paudioformat: ::core::option::Option<ITScriptableAudioFormat>) -> ::windows::core::Result<()>;
    fn EmptyAudioFormatForScripting(&mut self) -> ::windows::core::Result<ITScriptableAudioFormat>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITFileTrackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITFileTrackVtbl {
        unsafe extern "system" fn Format<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&pmt)).into()
        }
        unsafe extern "system" fn ControllingTerminal<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontrollingterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControllingTerminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontrollingterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatForScripting<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatForScripting() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaudioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioFormatForScripting<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudioformat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioFormatForScripting(::core::mem::transmute(&paudioformat)).into()
        }
        unsafe extern "system" fn EmptyAudioFormatForScripting<Impl: ITFileTrackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmptyAudioFormatForScripting() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaudioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Format: Format::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            ControllingTerminal: ControllingTerminal::<Impl, IMPL_OFFSET>,
            AudioFormatForScripting: AudioFormatForScripting::<Impl, IMPL_OFFSET>,
            SetAudioFormatForScripting: SetAudioFormatForScripting::<Impl, IMPL_OFFSET>,
            EmptyAudioFormatForScripting: EmptyAudioFormatForScripting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITFileTrack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITForwardInformationImpl: Sized + IDispatchImpl {
    fn SetNumRingsNoAnswer(&mut self, lnumrings: i32) -> ::windows::core::Result<()>;
    fn NumRingsNoAnswer(&mut self) -> ::windows::core::Result<i32>;
    fn SetForwardType(&mut self, forwardtype: i32, pdestaddress: super::super::Foundation::BSTR, pcalleraddress: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ForwardTypeDestination(&mut self, forwardtype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ForwardTypeCaller(&mut self, forwardtype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetForwardType(&mut self, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITForwardInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITForwardInformationVtbl {
        unsafe extern "system" fn SetNumRingsNoAnswer<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumrings: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumRingsNoAnswer(::core::mem::transmute_copy(&lnumrings)).into()
        }
        unsafe extern "system" fn NumRingsNoAnswer<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnumrings: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumRingsNoAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    *plnumrings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForwardType<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForwardType(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&pdestaddress), ::core::mem::transmute_copy(&pcalleraddress)).into()
        }
        unsafe extern "system" fn ForwardTypeDestination<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardTypeDestination(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeCaller<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardTypeCaller(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcalleraddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForwardType<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForwardType(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&ppcalleraddress)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITForwardInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetNumRingsNoAnswer: SetNumRingsNoAnswer::<Impl, IMPL_OFFSET>,
            NumRingsNoAnswer: NumRingsNoAnswer::<Impl, IMPL_OFFSET>,
            SetForwardType: SetForwardType::<Impl, IMPL_OFFSET>,
            ForwardTypeDestination: ForwardTypeDestination::<Impl, IMPL_OFFSET>,
            ForwardTypeCaller: ForwardTypeCaller::<Impl, IMPL_OFFSET>,
            GetForwardType: GetForwardType::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITForwardInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITForwardInformation2Impl: Sized + IDispatchImpl + ITForwardInformationImpl {
    fn SetForwardType2(&mut self, forwardtype: i32, pdestaddress: super::super::Foundation::BSTR, destaddresstype: i32, pcalleraddress: super::super::Foundation::BSTR, calleraddresstype: i32) -> ::windows::core::Result<()>;
    fn GetForwardType2(&mut self, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut super::super::Foundation::BSTR, pcalleraddresstype: *mut i32) -> ::windows::core::Result<()>;
    fn ForwardTypeDestinationAddressType(&mut self, forwardtype: i32) -> ::windows::core::Result<i32>;
    fn ForwardTypeCallerAddressType(&mut self, forwardtype: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITForwardInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITForwardInformation2Vtbl {
        unsafe extern "system" fn SetForwardType2<Impl: ITForwardInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destaddresstype: i32, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, calleraddresstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForwardType2(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&pdestaddress), ::core::mem::transmute_copy(&destaddresstype), ::core::mem::transmute_copy(&pcalleraddress), ::core::mem::transmute_copy(&calleraddresstype)).into()
        }
        unsafe extern "system" fn GetForwardType2<Impl: ITForwardInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut super::super::Foundation::BSTR, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForwardType2(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&pdestaddresstype), ::core::mem::transmute_copy(&ppcalleraddress), ::core::mem::transmute_copy(&pcalleraddresstype)).into()
        }
        unsafe extern "system" fn ForwardTypeDestinationAddressType<Impl: ITForwardInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardTypeDestinationAddressType(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdestaddresstype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeCallerAddressType<Impl: ITForwardInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardTypeCallerAddressType(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcalleraddresstype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITForwardInformationVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetForwardType2: SetForwardType2::<Impl, IMPL_OFFSET>,
            GetForwardType2: GetForwardType2::<Impl, IMPL_OFFSET>,
            ForwardTypeDestinationAddressType: ForwardTypeDestinationAddressType::<Impl, IMPL_OFFSET>,
            ForwardTypeCallerAddressType: ForwardTypeCallerAddressType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITForwardInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITILSConfigImpl: Sized + IDispatchImpl {
    fn Port(&mut self) -> ::windows::core::Result<i32>;
    fn SetPort(&mut self, port: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITILSConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITILSConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITILSConfigVtbl {
        unsafe extern "system" fn Port<Impl: ITILSConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Port() {
                ::core::result::Result::Ok(ok__) => {
                    *pport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Impl: ITILSConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, port: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPort(::core::mem::transmute_copy(&port)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Port: Port::<Impl, IMPL_OFFSET>, SetPort: SetPort::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITILSConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITLegacyAddressMediaControlImpl: Sized {
    fn GetID(&mut self, pdeviceclass: super::super::Foundation::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetDevConfig(&mut self, pdeviceclass: super::super::Foundation::BSTR, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetDevConfig(&mut self, pdeviceclass: super::super::Foundation::BSTR, dwsize: u32, pdeviceconfig: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITLegacyAddressMediaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyAddressMediaControlVtbl {
        unsafe extern "system" fn GetID<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetID(::core::mem::transmute_copy(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn GetDevConfig<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevConfig(::core::mem::transmute_copy(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceconfig)).into()
        }
        unsafe extern "system" fn SetDevConfig<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsize: u32, pdeviceconfig: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDevConfig(::core::mem::transmute_copy(&pdeviceclass), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdeviceconfig)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetID: GetID::<Impl, IMPL_OFFSET>,
            GetDevConfig: GetDevConfig::<Impl, IMPL_OFFSET>,
            SetDevConfig: SetDevConfig::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyAddressMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITLegacyAddressMediaControl2Impl: Sized + ITLegacyAddressMediaControlImpl {
    fn ConfigDialog(&mut self, hwndowner: super::super::Foundation::HWND, pdeviceclass: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ConfigDialogEdit(&mut self, hwndowner: super::super::Foundation::HWND, pdeviceclass: super::super::Foundation::BSTR, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITLegacyAddressMediaControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyAddressMediaControl2Vtbl {
        unsafe extern "system" fn ConfigDialog<Impl: ITLegacyAddressMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigDialog(::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&pdeviceclass)).into()
        }
        unsafe extern "system" fn ConfigDialogEdit<Impl: ITLegacyAddressMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigDialogEdit(::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&pdeviceclass), ::core::mem::transmute_copy(&dwsizein), ::core::mem::transmute_copy(&pdeviceconfigin), ::core::mem::transmute_copy(&pdwsizeout), ::core::mem::transmute_copy(&ppdeviceconfigout)).into()
        }
        Self {
            base: ITLegacyAddressMediaControlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ConfigDialog: ConfigDialog::<Impl, IMPL_OFFSET>,
            ConfigDialogEdit: ConfigDialogEdit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyAddressMediaControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyCallMediaControlImpl: Sized + IDispatchImpl {
    fn DetectDigits(&mut self, digitmode: i32) -> ::windows::core::Result<()>;
    fn GenerateDigits(&mut self, pdigits: super::super::Foundation::BSTR, digitmode: i32) -> ::windows::core::Result<()>;
    fn GetID(&mut self, pdeviceclass: super::super::Foundation::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetMediaType(&mut self, lmediatype: i32) -> ::windows::core::Result<()>;
    fn MonitorMedia(&mut self, lmediatype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyCallMediaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyCallMediaControlVtbl {
        unsafe extern "system" fn DetectDigits<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectDigits(::core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GenerateDigits<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateDigits(::core::mem::transmute_copy(&pdigits), ::core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GetID<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetID(::core::mem::transmute_copy(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn SetMediaType<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaType(::core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn MonitorMedia<Impl: ITLegacyCallMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MonitorMedia(::core::mem::transmute_copy(&lmediatype)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DetectDigits: DetectDigits::<Impl, IMPL_OFFSET>,
            GenerateDigits: GenerateDigits::<Impl, IMPL_OFFSET>,
            GetID: GetID::<Impl, IMPL_OFFSET>,
            SetMediaType: SetMediaType::<Impl, IMPL_OFFSET>,
            MonitorMedia: MonitorMedia::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyCallMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyCallMediaControl2Impl: Sized + IDispatchImpl + ITLegacyCallMediaControlImpl {
    fn GenerateDigits2(&mut self, pdigits: super::super::Foundation::BSTR, digitmode: i32, lduration: i32) -> ::windows::core::Result<()>;
    fn GatherDigits(&mut self, digitmode: i32, lnumdigits: i32, pterminationdigits: super::super::Foundation::BSTR, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::Result<()>;
    fn DetectTones(&mut self, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::Result<()>;
    fn DetectTonesByCollection(&mut self, pdetecttonecollection: ::core::option::Option<ITCollection2>) -> ::windows::core::Result<()>;
    fn GenerateTone(&mut self, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::Result<()>;
    fn GenerateCustomTones(&mut self, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::Result<()>;
    fn GenerateCustomTonesByCollection(&mut self, pcustomtonecollection: ::core::option::Option<ITCollection2>, lduration: i32) -> ::windows::core::Result<()>;
    fn CreateDetectToneObject(&mut self) -> ::windows::core::Result<ITDetectTone>;
    fn CreateCustomToneObject(&mut self) -> ::windows::core::Result<ITCustomTone>;
    fn GetIDAsVariant(&mut self, bstrdeviceclass: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyCallMediaControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyCallMediaControl2Vtbl {
        unsafe extern "system" fn GenerateDigits2<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateDigits2(::core::mem::transmute_copy(&pdigits), ::core::mem::transmute_copy(&digitmode), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GatherDigits<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitmode: i32, lnumdigits: i32, pterminationdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GatherDigits(::core::mem::transmute_copy(&digitmode), ::core::mem::transmute_copy(&lnumdigits), ::core::mem::transmute_copy(&pterminationdigits), ::core::mem::transmute_copy(&lfirstdigittimeout), ::core::mem::transmute_copy(&linterdigittimeout)).into()
        }
        unsafe extern "system" fn DetectTones<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectTones(::core::mem::transmute_copy(&ptonelist), ::core::mem::transmute_copy(&lnumtones)).into()
        }
        unsafe extern "system" fn DetectTonesByCollection<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetecttonecollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectTonesByCollection(::core::mem::transmute(&pdetecttonecollection)).into()
        }
        unsafe extern "system" fn GenerateTone<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateTone(::core::mem::transmute_copy(&tonemode), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTones<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateCustomTones(::core::mem::transmute_copy(&ptonelist), ::core::mem::transmute_copy(&lnumtones), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTonesByCollection<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustomtonecollection: ::windows::core::RawPtr, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateCustomTonesByCollection(::core::mem::transmute(&pcustomtonecollection), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn CreateDetectToneObject<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdetecttone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDetectToneObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdetecttone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomToneObject<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcustomtone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomToneObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcustomtone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDAsVariant<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardeviceid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIDAsVariant(::core::mem::transmute_copy(&bstrdeviceclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvardeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITLegacyCallMediaControlVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GenerateDigits2: GenerateDigits2::<Impl, IMPL_OFFSET>,
            GatherDigits: GatherDigits::<Impl, IMPL_OFFSET>,
            DetectTones: DetectTones::<Impl, IMPL_OFFSET>,
            DetectTonesByCollection: DetectTonesByCollection::<Impl, IMPL_OFFSET>,
            GenerateTone: GenerateTone::<Impl, IMPL_OFFSET>,
            GenerateCustomTones: GenerateCustomTones::<Impl, IMPL_OFFSET>,
            GenerateCustomTonesByCollection: GenerateCustomTonesByCollection::<Impl, IMPL_OFFSET>,
            CreateDetectToneObject: CreateDetectToneObject::<Impl, IMPL_OFFSET>,
            CreateCustomToneObject: CreateCustomToneObject::<Impl, IMPL_OFFSET>,
            GetIDAsVariant: GetIDAsVariant::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyCallMediaControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyWaveSupportImpl: Sized + IDispatchImpl {
    fn IsFullDuplex(&mut self) -> ::windows::core::Result<FULLDUPLEX_SUPPORT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyWaveSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyWaveSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyWaveSupportVtbl {
        unsafe extern "system" fn IsFullDuplex<Impl: ITLegacyWaveSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupport: *mut FULLDUPLEX_SUPPORT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFullDuplex() {
                ::core::result::Result::Ok(ok__) => {
                    *psupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), IsFullDuplex: IsFullDuplex::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyWaveSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLocationInfoImpl: Sized + IDispatchImpl {
    fn PermanentLocationID(&mut self) -> ::windows::core::Result<i32>;
    fn CountryCode(&mut self) -> ::windows::core::Result<i32>;
    fn CountryID(&mut self) -> ::windows::core::Result<i32>;
    fn Options(&mut self) -> ::windows::core::Result<i32>;
    fn PreferredCardID(&mut self) -> ::windows::core::Result<i32>;
    fn LocationName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CityCode(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LocalAccessCode(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LongDistanceAccessCode(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TollPrefixList(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CancelCallWaitingCode(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLocationInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLocationInfoVtbl {
        unsafe extern "system" fn PermanentLocationID<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllocationid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermanentLocationID() {
                ::core::result::Result::Ok(ok__) => {
                    *pllocationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plcountrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryID<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountryid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryID() {
                ::core::result::Result::Ok(ok__) => {
                    *plcountryid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *ploptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredCardID<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredCardID() {
                ::core::result::Result::Ok(ok__) => {
                    *plcardid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationName<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplocationname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationName() {
                ::core::result::Result::Ok(ok__) => {
                    *pplocationname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CityCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CityCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAccessCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAccessCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceAccessCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongDistanceAccessCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TollPrefixList<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptolllist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TollPrefixList() {
                ::core::result::Result::Ok(ok__) => {
                    *pptolllist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelCallWaitingCode<Impl: ITLocationInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelCallWaitingCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PermanentLocationID: PermanentLocationID::<Impl, IMPL_OFFSET>,
            CountryCode: CountryCode::<Impl, IMPL_OFFSET>,
            CountryID: CountryID::<Impl, IMPL_OFFSET>,
            Options: Options::<Impl, IMPL_OFFSET>,
            PreferredCardID: PreferredCardID::<Impl, IMPL_OFFSET>,
            LocationName: LocationName::<Impl, IMPL_OFFSET>,
            CityCode: CityCode::<Impl, IMPL_OFFSET>,
            LocalAccessCode: LocalAccessCode::<Impl, IMPL_OFFSET>,
            LongDistanceAccessCode: LongDistanceAccessCode::<Impl, IMPL_OFFSET>,
            TollPrefixList: TollPrefixList::<Impl, IMPL_OFFSET>,
            CancelCallWaitingCode: CancelCallWaitingCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLocationInfo as ::windows::core::Interface>::IID
    }
}
pub trait ITMSPAddressImpl: Sized {
    fn Initialize(&mut self, hevent: *const i32) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
    fn CreateMSPCall(&mut self, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ShutdownMSPCall(&mut self, pstreamcontrol: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ReceiveTSPData(&mut self, pmspcall: ::core::option::Option<::windows::core::IUnknown>, pbuffer: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn GetEvent(&mut self, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::Result<()>;
}
impl ITMSPAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMSPAddressVtbl {
        unsafe extern "system" fn Initialize<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Shutdown<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn CreateMSPCall<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut ::core::ffi::c_void, ppstreamcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMSPCall(::core::mem::transmute_copy(&hcall), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&dwmediatype), ::core::mem::transmute(&pouterunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstreamcontrol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownMSPCall<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstreamcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutdownMSPCall(::core::mem::transmute(&pstreamcontrol)).into()
        }
        unsafe extern "system" fn ReceiveTSPData<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmspcall: *mut ::core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReceiveTSPData(::core::mem::transmute(&pmspcall), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetEvent<Impl: ITMSPAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEvent(::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&peventbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
            CreateMSPCall: CreateMSPCall::<Impl, IMPL_OFFSET>,
            ShutdownMSPCall: ShutdownMSPCall::<Impl, IMPL_OFFSET>,
            ReceiveTSPData: ReceiveTSPData::<Impl, IMPL_OFFSET>,
            GetEvent: GetEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMSPAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaControlImpl: Sized + IDispatchImpl {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn MediaState(&mut self) -> ::windows::core::Result<TERMINAL_MEDIA_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaControlVtbl {
        unsafe extern "system" fn Start<Impl: ITMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: ITMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Pause<Impl: ITMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn MediaState<Impl: ITMediaControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaState() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminalmediastate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            MediaState: MediaState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaPlaybackImpl: Sized + IDispatchImpl {
    fn SetPlayList(&mut self, playlistvariant: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlayList(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaPlaybackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaPlaybackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaPlaybackVtbl {
        unsafe extern "system" fn SetPlayList<Impl: ITMediaPlaybackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, playlistvariant: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayList(::core::mem::transmute_copy(&playlistvariant)).into()
        }
        unsafe extern "system" fn PlayList<Impl: ITMediaPlaybackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylistvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlayList() {
                ::core::result::Result::Ok(ok__) => {
                    *pplaylistvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPlayList: SetPlayList::<Impl, IMPL_OFFSET>,
            PlayList: PlayList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaPlayback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaRecordImpl: Sized + IDispatchImpl {
    fn SetFileName(&mut self, bstrfilename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaRecordVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaRecordImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaRecordVtbl {
        unsafe extern "system" fn SetFileName<Impl: ITMediaRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileName(::core::mem::transmute_copy(&bstrfilename)).into()
        }
        unsafe extern "system" fn FileName<Impl: ITMediaRecordImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetFileName: SetFileName::<Impl, IMPL_OFFSET>,
            FileName: FileName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaRecord as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaSupportImpl: Sized + IDispatchImpl {
    fn MediaTypes(&mut self) -> ::windows::core::Result<i32>;
    fn QueryMediaType(&mut self, lmediatype: i32) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaSupportVtbl {
        unsafe extern "system" fn MediaTypes<Impl: ITMediaSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMediaType<Impl: ITMediaSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, pfsupport: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMediaType(::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MediaTypes: MediaTypes::<Impl, IMPL_OFFSET>,
            QueryMediaType: QueryMediaType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMultiTrackTerminalImpl: Sized + IDispatchImpl {
    fn TrackTerminals(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateTrackTerminals(&mut self) -> ::windows::core::Result<IEnumTerminal>;
    fn CreateTrackTerminal(&mut self, mediatype: i32, terminaldirection: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
    fn MediaTypesInUse(&mut self) -> ::windows::core::Result<i32>;
    fn DirectionsInUse(&mut self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
    fn RemoveTrackTerminal(&mut self, ptrackterminaltoremove: ::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMultiTrackTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMultiTrackTerminalVtbl {
        unsafe extern "system" fn TrackTerminals<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTrackTerminals<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTrackTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrackTerminal<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTrackTerminal(::core::mem::transmute_copy(&mediatype), ::core::mem::transmute_copy(&terminaldirection)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypesInUse<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypesinuse: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaTypesInUse() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypesinuse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectionsInUse<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldirectionsinused: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectionsInUse() {
                ::core::result::Result::Ok(ok__) => {
                    *pldirectionsinused = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrackTerminal<Impl: ITMultiTrackTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrackterminaltoremove: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTrackTerminal(::core::mem::transmute(&ptrackterminaltoremove)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TrackTerminals: TrackTerminals::<Impl, IMPL_OFFSET>,
            EnumerateTrackTerminals: EnumerateTrackTerminals::<Impl, IMPL_OFFSET>,
            CreateTrackTerminal: CreateTrackTerminal::<Impl, IMPL_OFFSET>,
            MediaTypesInUse: MediaTypesInUse::<Impl, IMPL_OFFSET>,
            DirectionsInUse: DirectionsInUse::<Impl, IMPL_OFFSET>,
            RemoveTrackTerminal: RemoveTrackTerminal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMultiTrackTerminal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneImpl: Sized + IDispatchImpl {
    fn Open(&mut self, privilege: PHONE_PRIVILEGE) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Addresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateAddresses(&mut self) -> ::windows::core::Result<IEnumAddress>;
    fn PhoneCapsLong(&mut self, pclcap: PHONECAPS_LONG) -> ::windows::core::Result<i32>;
    fn PhoneCapsString(&mut self, pcscap: PHONECAPS_STRING) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Terminals(&mut self, paddress: ::core::option::Option<ITAddress>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateTerminals(&mut self, paddress: ::core::option::Option<ITAddress>) -> ::windows::core::Result<IEnumTerminal>;
    fn ButtonMode(&mut self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_MODE>;
    fn SetButtonMode(&mut self, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::Result<()>;
    fn ButtonFunction(&mut self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_FUNCTION>;
    fn SetButtonFunction(&mut self, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::Result<()>;
    fn ButtonText(&mut self, lbuttonid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetButtonText(&mut self, lbuttonid: i32, bstrbuttontext: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ButtonState(&mut self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_STATE>;
    fn HookSwitchState(&mut self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE) -> ::windows::core::Result<PHONE_HOOK_SWITCH_STATE>;
    fn SetHookSwitchState(&mut self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows::core::Result<()>;
    fn SetRingMode(&mut self, lringmode: i32) -> ::windows::core::Result<()>;
    fn RingMode(&mut self) -> ::windows::core::Result<i32>;
    fn SetRingVolume(&mut self, lringvolume: i32) -> ::windows::core::Result<()>;
    fn RingVolume(&mut self) -> ::windows::core::Result<i32>;
    fn Privilege(&mut self) -> ::windows::core::Result<PHONE_PRIVILEGE>;
    fn GetPhoneCapsBuffer(&mut self, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows::core::Result<()>;
    fn PhoneCapsBuffer(&mut self, pcbcaps: PHONECAPS_BUFFER) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn LampMode(&mut self, llampid: i32) -> ::windows::core::Result<PHONE_LAMP_MODE>;
    fn SetLampMode(&mut self, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows::core::Result<()>;
    fn Display(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplay(&mut self, lrow: i32, lcolumn: i32, bstrdisplay: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PreferredAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePreferredAddresses(&mut self) -> ::windows::core::Result<IEnumAddress>;
    fn DeviceSpecific(&mut self, pparams: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn DeviceSpecificVariant(&mut self, vardevspecificbytearray: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NegotiateExtVersion(&mut self, llowversion: i32, lhighversion: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhoneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPhoneVtbl {
        unsafe extern "system" fn Open<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privilege: PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&privilege)).into()
        }
        unsafe extern "system" fn Close<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Addresses<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *paddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneCapsLong<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneCapsLong(::core::mem::transmute_copy(&pclcap)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneCapsString<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcscap: PHONECAPS_STRING, ppcapability: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneCapsString(::core::mem::transmute_copy(&pcscap)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminals(::core::mem::transmute(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *pterminals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTerminals(::core::mem::transmute(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonMode(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbuttonmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonMode(::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&buttonmode)).into()
        }
        unsafe extern "system" fn ButtonFunction<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonFunction(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbuttonfunction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonFunction<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonFunction(::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&buttonfunction)).into()
        }
        unsafe extern "system" fn ButtonText<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, ppbuttontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonText(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuttontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonText<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, bstrbuttontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonText(::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&bstrbuttontext)).into()
        }
        unsafe extern "system" fn ButtonState<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonState(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbuttonstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HookSwitchState(::core::mem::transmute_copy(&hookswitchdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *phookswitchstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHookSwitchState<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHookSwitchState(::core::mem::transmute_copy(&hookswitchdevice), ::core::mem::transmute_copy(&hookswitchstate)).into()
        }
        unsafe extern "system" fn SetRingMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRingMode(::core::mem::transmute_copy(&lringmode)).into()
        }
        unsafe extern "system" fn RingMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plringmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingVolume<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRingVolume(::core::mem::transmute_copy(&lringvolume)).into()
        }
        unsafe extern "system" fn RingVolume<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *plringvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivilege: *mut PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Privilege() {
                ::core::result::Result::Ok(ok__) => {
                    *pprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneCapsBuffer<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPhoneCapsBuffer(::core::mem::transmute_copy(&pcbcaps), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppphonecapsbuffer)).into()
        }
        unsafe extern "system" fn PhoneCapsBuffer<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneCapsBuffer(::core::mem::transmute_copy(&pcbcaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LampMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LampMode(::core::mem::transmute_copy(&llampid)) {
                ::core::result::Result::Ok(ok__) => {
                    *plampmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLampMode<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLampMode(::core::mem::transmute_copy(&llampid), ::core::mem::transmute_copy(&lampmode)).into()
        }
        unsafe extern "system" fn Display<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisplay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplay<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrow: i32, lcolumn: i32, bstrdisplay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplay(::core::mem::transmute_copy(&lrow), ::core::mem::transmute_copy(&lcolumn), ::core::mem::transmute_copy(&bstrdisplay)).into()
        }
        unsafe extern "system" fn PreferredAddresses<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *paddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredAddresses<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePreferredAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSpecific<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSpecific(::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSpecificVariant(::core::mem::transmute_copy(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Impl: ITPhoneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegotiateExtVersion(::core::mem::transmute_copy(&llowversion), ::core::mem::transmute_copy(&lhighversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *plextversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Addresses: Addresses::<Impl, IMPL_OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Impl, IMPL_OFFSET>,
            PhoneCapsLong: PhoneCapsLong::<Impl, IMPL_OFFSET>,
            PhoneCapsString: PhoneCapsString::<Impl, IMPL_OFFSET>,
            Terminals: Terminals::<Impl, IMPL_OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Impl, IMPL_OFFSET>,
            ButtonMode: ButtonMode::<Impl, IMPL_OFFSET>,
            SetButtonMode: SetButtonMode::<Impl, IMPL_OFFSET>,
            ButtonFunction: ButtonFunction::<Impl, IMPL_OFFSET>,
            SetButtonFunction: SetButtonFunction::<Impl, IMPL_OFFSET>,
            ButtonText: ButtonText::<Impl, IMPL_OFFSET>,
            SetButtonText: SetButtonText::<Impl, IMPL_OFFSET>,
            ButtonState: ButtonState::<Impl, IMPL_OFFSET>,
            HookSwitchState: HookSwitchState::<Impl, IMPL_OFFSET>,
            SetHookSwitchState: SetHookSwitchState::<Impl, IMPL_OFFSET>,
            SetRingMode: SetRingMode::<Impl, IMPL_OFFSET>,
            RingMode: RingMode::<Impl, IMPL_OFFSET>,
            SetRingVolume: SetRingVolume::<Impl, IMPL_OFFSET>,
            RingVolume: RingVolume::<Impl, IMPL_OFFSET>,
            Privilege: Privilege::<Impl, IMPL_OFFSET>,
            GetPhoneCapsBuffer: GetPhoneCapsBuffer::<Impl, IMPL_OFFSET>,
            PhoneCapsBuffer: PhoneCapsBuffer::<Impl, IMPL_OFFSET>,
            LampMode: LampMode::<Impl, IMPL_OFFSET>,
            SetLampMode: SetLampMode::<Impl, IMPL_OFFSET>,
            Display: Display::<Impl, IMPL_OFFSET>,
            SetDisplay: SetDisplay::<Impl, IMPL_OFFSET>,
            PreferredAddresses: PreferredAddresses::<Impl, IMPL_OFFSET>,
            EnumeratePreferredAddresses: EnumeratePreferredAddresses::<Impl, IMPL_OFFSET>,
            DeviceSpecific: DeviceSpecific::<Impl, IMPL_OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Impl, IMPL_OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhone as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneDeviceSpecificEventImpl: Sized + IDispatchImpl {
    fn Phone(&mut self) -> ::windows::core::Result<ITPhone>;
    fn lParam1(&mut self) -> ::windows::core::Result<i32>;
    fn lParam2(&mut self) -> ::windows::core::Result<i32>;
    fn lParam3(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhoneDeviceSpecificEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneDeviceSpecificEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPhoneDeviceSpecificEventVtbl {
        unsafe extern "system" fn Phone<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam1() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam2() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam3() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam3 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Phone: Phone::<Impl, IMPL_OFFSET>,
            lParam1: lParam1::<Impl, IMPL_OFFSET>,
            lParam2: lParam2::<Impl, IMPL_OFFSET>,
            lParam3: lParam3::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhoneDeviceSpecificEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneEventImpl: Sized + IDispatchImpl {
    fn Phone(&mut self) -> ::windows::core::Result<ITPhone>;
    fn Event(&mut self) -> ::windows::core::Result<PHONE_EVENT>;
    fn ButtonState(&mut self) -> ::windows::core::Result<PHONE_BUTTON_STATE>;
    fn HookSwitchState(&mut self) -> ::windows::core::Result<PHONE_HOOK_SWITCH_STATE>;
    fn HookSwitchDevice(&mut self) -> ::windows::core::Result<PHONE_HOOK_SWITCH_DEVICE>;
    fn RingMode(&mut self) -> ::windows::core::Result<i32>;
    fn ButtonLampId(&mut self) -> ::windows::core::Result<i32>;
    fn NumberGathered(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhoneEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPhoneEventVtbl {
        unsafe extern "system" fn Phone<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut PHONE_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonState<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HookSwitchState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchDevice<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HookSwitchDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *pdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingMode<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plringmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonLampId<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbuttonlampid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonLampId() {
                ::core::result::Result::Ok(ok__) => {
                    *plbuttonlampid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberGathered<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberGathered() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITPhoneEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Phone: Phone::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            ButtonState: ButtonState::<Impl, IMPL_OFFSET>,
            HookSwitchState: HookSwitchState::<Impl, IMPL_OFFSET>,
            HookSwitchDevice: HookSwitchDevice::<Impl, IMPL_OFFSET>,
            RingMode: RingMode::<Impl, IMPL_OFFSET>,
            ButtonLampId: ButtonLampId::<Impl, IMPL_OFFSET>,
            NumberGathered: NumberGathered::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhoneEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPluggableTerminalClassInfoImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Company(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Version(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TerminalClass(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CLSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Direction(&mut self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
    fn MediaTypes(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPluggableTerminalClassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalClassInfoVtbl {
        unsafe extern "system" fn Name<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Company<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompany: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Company() {
                ::core::result::Result::Ok(ok__) => {
                    *pcompany = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *pversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalClass() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminalclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypes<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Company: Company::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            TerminalClass: TerminalClass::<Impl, IMPL_OFFSET>,
            CLSID: CLSID::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            MediaTypes: MediaTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalClassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalEventSinkImpl: Sized {
    fn FireEvent(&mut self, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalEventSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalEventSinkVtbl {
        unsafe extern "system" fn FireEvent<Impl: ITPluggableTerminalEventSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FireEvent(::core::mem::transmute_copy(&pmspeventinfo)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FireEvent: FireEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalEventSink as ::windows::core::Interface>::IID
    }
}
pub trait ITPluggableTerminalEventSinkRegistrationImpl: Sized {
    fn RegisterSink(&mut self, peventsink: ::core::option::Option<ITPluggableTerminalEventSink>) -> ::windows::core::Result<()>;
    fn UnregisterSink(&mut self) -> ::windows::core::Result<()>;
}
impl ITPluggableTerminalEventSinkRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSinkRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalEventSinkRegistrationVtbl {
        unsafe extern "system" fn RegisterSink<Impl: ITPluggableTerminalEventSinkRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterSink(::core::mem::transmute(&peventsink)).into()
        }
        unsafe extern "system" fn UnregisterSink<Impl: ITPluggableTerminalEventSinkRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterSink().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterSink: RegisterSink::<Impl, IMPL_OFFSET>,
            UnregisterSink: UnregisterSink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalEventSinkRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPluggableTerminalSuperclassInfoImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CLSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPluggableTerminalSuperclassInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalSuperclassInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalSuperclassInfoVtbl {
        unsafe extern "system" fn Name<Impl: ITPluggableTerminalSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Impl: ITPluggableTerminalSuperclassInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Name: Name::<Impl, IMPL_OFFSET>, CLSID: CLSID::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalSuperclassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPrivateEventImpl: Sized + IDispatchImpl {
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn CallHub(&mut self) -> ::windows::core::Result<ITCallHub>;
    fn EventCode(&mut self) -> ::windows::core::Result<i32>;
    fn EventInterface(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPrivateEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPrivateEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPrivateEventVtbl {
        unsafe extern "system" fn Address<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHub() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventCode<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pleventcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pleventcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterface<Impl: ITPrivateEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *peventinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Address: Address::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            CallHub: CallHub::<Impl, IMPL_OFFSET>,
            EventCode: EventCode::<Impl, IMPL_OFFSET>,
            EventInterface: EventInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPrivateEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQOSEventImpl: Sized + IDispatchImpl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Event(&mut self) -> ::windows::core::Result<QOS_EVENT>;
    fn MediaType(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQOSEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQOSEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITQOSEventVtbl {
        unsafe extern "system" fn Call<Impl: ITQOSEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITQOSEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqosevent: *mut QOS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pqosevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: ITQOSEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQOSEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQueueImpl: Sized + IDispatchImpl {
    fn SetMeasurementPeriod(&mut self, lperiod: i32) -> ::windows::core::Result<()>;
    fn MeasurementPeriod(&mut self) -> ::windows::core::Result<i32>;
    fn TotalCallsQueued(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentCallsQueued(&mut self) -> ::windows::core::Result<i32>;
    fn TotalCallsAbandoned(&mut self) -> ::windows::core::Result<i32>;
    fn TotalCallsFlowedIn(&mut self) -> ::windows::core::Result<i32>;
    fn TotalCallsFlowedOut(&mut self) -> ::windows::core::Result<i32>;
    fn LongestEverWaitTime(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentLongestWaitTime(&mut self) -> ::windows::core::Result<i32>;
    fn AverageWaitTime(&mut self) -> ::windows::core::Result<i32>;
    fn FinalDisposition(&mut self) -> ::windows::core::Result<i32>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITQueueVtbl {
        unsafe extern "system" fn SetMeasurementPeriod<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMeasurementPeriod(::core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasurementPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *plperiod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsQueued<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallsQueued() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCallsQueued<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCallsQueued() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsAbandoned<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallsAbandoned() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedIn<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallsFlowedIn() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedOut<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallsFlowedOut() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestEverWaitTime<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongestEverWaitTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLongestWaitTime<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLongestWaitTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWaitTime<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AverageWaitTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalDisposition<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalDisposition() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMeasurementPeriod: SetMeasurementPeriod::<Impl, IMPL_OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Impl, IMPL_OFFSET>,
            TotalCallsQueued: TotalCallsQueued::<Impl, IMPL_OFFSET>,
            CurrentCallsQueued: CurrentCallsQueued::<Impl, IMPL_OFFSET>,
            TotalCallsAbandoned: TotalCallsAbandoned::<Impl, IMPL_OFFSET>,
            TotalCallsFlowedIn: TotalCallsFlowedIn::<Impl, IMPL_OFFSET>,
            TotalCallsFlowedOut: TotalCallsFlowedOut::<Impl, IMPL_OFFSET>,
            LongestEverWaitTime: LongestEverWaitTime::<Impl, IMPL_OFFSET>,
            CurrentLongestWaitTime: CurrentLongestWaitTime::<Impl, IMPL_OFFSET>,
            AverageWaitTime: AverageWaitTime::<Impl, IMPL_OFFSET>,
            FinalDisposition: FinalDisposition::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQueueEventImpl: Sized + IDispatchImpl {
    fn Queue(&mut self) -> ::windows::core::Result<ITQueue>;
    fn Event(&mut self) -> ::windows::core::Result<ACDQUEUE_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQueueEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQueueEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITQueueEventVtbl {
        unsafe extern "system" fn Queue<Impl: ITQueueEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Queue() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITQueueEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDQUEUE_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Queue: Queue::<Impl, IMPL_OFFSET>, Event: Event::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQueueEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRendezvousImpl: Sized + IDispatchImpl {
    fn DefaultDirectories(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDefaultDirectories(&mut self) -> ::windows::core::Result<IEnumDirectory>;
    fn CreateDirectory(&mut self, directorytype: DIRECTORY_TYPE, pname: super::super::Foundation::BSTR) -> ::windows::core::Result<ITDirectory>;
    fn CreateDirectoryObject(&mut self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: super::super::Foundation::BSTR) -> ::windows::core::Result<ITDirectoryObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRendezvousVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRendezvousImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITRendezvousVtbl {
        unsafe extern "system" fn DefaultDirectories<Impl: ITRendezvousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultDirectories() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDefaultDirectories<Impl: ITRendezvousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdirectory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateDefaultDirectories() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdirectory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectory<Impl: ITRendezvousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directorytype: DIRECTORY_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirectory(::core::mem::transmute_copy(&directorytype), ::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectoryObject<Impl: ITRendezvousImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdirectoryobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirectoryObject(::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdirectoryobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DefaultDirectories: DefaultDirectories::<Impl, IMPL_OFFSET>,
            EnumerateDefaultDirectories: EnumerateDefaultDirectories::<Impl, IMPL_OFFSET>,
            CreateDirectory: CreateDirectory::<Impl, IMPL_OFFSET>,
            CreateDirectoryObject: CreateDirectoryObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRendezvous as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRequestImpl: Sized + IDispatchImpl {
    fn MakeCall(&mut self, pdestaddress: super::super::Foundation::BSTR, pappname: super::super::Foundation::BSTR, pcalledparty: super::super::Foundation::BSTR, pcomment: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITRequestVtbl {
        unsafe extern "system" fn MakeCall<Impl: ITRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalledparty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeCall(::core::mem::transmute_copy(&pdestaddress), ::core::mem::transmute_copy(&pappname), ::core::mem::transmute_copy(&pcalledparty), ::core::mem::transmute_copy(&pcomment)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), MakeCall: MakeCall::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRequestEventImpl: Sized + IDispatchImpl {
    fn RegistrationInstance(&mut self) -> ::windows::core::Result<i32>;
    fn RequestMode(&mut self) -> ::windows::core::Result<i32>;
    fn DestAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AppName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CalledParty(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Comment(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRequestEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITRequestEventVtbl {
        unsafe extern "system" fn RegistrationInstance<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plregistrationinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistrationInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plregistrationinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMode<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrequestmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plrequestmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestAddress<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppName<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppappname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalledParty<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcalledparty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalledParty() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcalledparty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Impl: ITRequestEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegistrationInstance: RegistrationInstance::<Impl, IMPL_OFFSET>,
            RequestMode: RequestMode::<Impl, IMPL_OFFSET>,
            DestAddress: DestAddress::<Impl, IMPL_OFFSET>,
            AppName: AppName::<Impl, IMPL_OFFSET>,
            CalledParty: CalledParty::<Impl, IMPL_OFFSET>,
            Comment: Comment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRequestEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITScriptableAudioFormatImpl: Sized + IDispatchImpl {
    fn Channels(&mut self) -> ::windows::core::Result<i32>;
    fn SetChannels(&mut self, nnewval: i32) -> ::windows::core::Result<()>;
    fn SamplesPerSec(&mut self) -> ::windows::core::Result<i32>;
    fn SetSamplesPerSec(&mut self, nnewval: i32) -> ::windows::core::Result<()>;
    fn AvgBytesPerSec(&mut self) -> ::windows::core::Result<i32>;
    fn SetAvgBytesPerSec(&mut self, nnewval: i32) -> ::windows::core::Result<()>;
    fn BlockAlign(&mut self) -> ::windows::core::Result<i32>;
    fn SetBlockAlign(&mut self, nnewval: i32) -> ::windows::core::Result<()>;
    fn BitsPerSample(&mut self) -> ::windows::core::Result<i32>;
    fn SetBitsPerSample(&mut self, nnewval: i32) -> ::windows::core::Result<()>;
    fn FormatTag(&mut self) -> ::windows::core::Result<i32>;
    fn SetFormatTag(&mut self, nnewval: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITScriptableAudioFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITScriptableAudioFormatVtbl {
        unsafe extern "system" fn Channels<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channels() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannels(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn SamplesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SamplesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSamplesPerSec(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn AvgBytesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvgBytesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAvgBytesPerSec(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BlockAlign<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockAlign() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlockAlign(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BitsPerSample<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitsPerSample() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitsPerSample(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn FormatTag<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatTag() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Impl: ITScriptableAudioFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatTag(::core::mem::transmute_copy(&nnewval)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Channels: Channels::<Impl, IMPL_OFFSET>,
            SetChannels: SetChannels::<Impl, IMPL_OFFSET>,
            SamplesPerSec: SamplesPerSec::<Impl, IMPL_OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Impl, IMPL_OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Impl, IMPL_OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Impl, IMPL_OFFSET>,
            BlockAlign: BlockAlign::<Impl, IMPL_OFFSET>,
            SetBlockAlign: SetBlockAlign::<Impl, IMPL_OFFSET>,
            BitsPerSample: BitsPerSample::<Impl, IMPL_OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Impl, IMPL_OFFSET>,
            FormatTag: FormatTag::<Impl, IMPL_OFFSET>,
            SetFormatTag: SetFormatTag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITScriptableAudioFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStaticAudioTerminalImpl: Sized + IDispatchImpl {
    fn WaveId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStaticAudioTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStaticAudioTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITStaticAudioTerminalVtbl {
        unsafe extern "system" fn WaveId<Impl: ITStaticAudioTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaveId() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaveid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), WaveId: WaveId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStaticAudioTerminal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStreamImpl: Sized + IDispatchImpl {
    fn MediaType(&mut self) -> ::windows::core::Result<i32>;
    fn Direction(&mut self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StartStream(&mut self) -> ::windows::core::Result<()>;
    fn PauseStream(&mut self) -> ::windows::core::Result<()>;
    fn StopStream(&mut self) -> ::windows::core::Result<()>;
    fn SelectTerminal(&mut self, pterminal: ::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn UnselectTerminal(&mut self, pterminal: ::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn EnumerateTerminals(&mut self) -> ::windows::core::Result<IEnumTerminal>;
    fn Terminals(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITStreamVtbl {
        unsafe extern "system" fn MediaType<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptd: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartStream<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartStream().into()
        }
        unsafe extern "system" fn PauseStream<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseStream().into()
        }
        unsafe extern "system" fn StopStream<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopStream().into()
        }
        unsafe extern "system" fn SelectTerminal<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnselectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Impl: ITStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            StartStream: StartStream::<Impl, IMPL_OFFSET>,
            PauseStream: PauseStream::<Impl, IMPL_OFFSET>,
            StopStream: StopStream::<Impl, IMPL_OFFSET>,
            SelectTerminal: SelectTerminal::<Impl, IMPL_OFFSET>,
            UnselectTerminal: UnselectTerminal::<Impl, IMPL_OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Impl, IMPL_OFFSET>,
            Terminals: Terminals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStreamControlImpl: Sized + IDispatchImpl {
    fn CreateStream(&mut self, lmediatype: i32, td: TERMINAL_DIRECTION) -> ::windows::core::Result<ITStream>;
    fn RemoveStream(&mut self, pstream: ::core::option::Option<ITStream>) -> ::windows::core::Result<()>;
    fn EnumerateStreams(&mut self) -> ::windows::core::Result<IEnumStream>;
    fn Streams(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStreamControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITStreamControlVtbl {
        unsafe extern "system" fn CreateStream<Impl: ITStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStream(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&td)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Impl: ITStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn EnumerateStreams<Impl: ITStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Streams<Impl: ITStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Streams() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateStream: CreateStream::<Impl, IMPL_OFFSET>,
            RemoveStream: RemoveStream::<Impl, IMPL_OFFSET>,
            EnumerateStreams: EnumerateStreams::<Impl, IMPL_OFFSET>,
            Streams: Streams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStreamControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITSubStreamImpl: Sized + IDispatchImpl {
    fn StartSubStream(&mut self) -> ::windows::core::Result<()>;
    fn PauseSubStream(&mut self) -> ::windows::core::Result<()>;
    fn StopSubStream(&mut self) -> ::windows::core::Result<()>;
    fn SelectTerminal(&mut self, pterminal: ::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn UnselectTerminal(&mut self, pterminal: ::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn EnumerateTerminals(&mut self) -> ::windows::core::Result<IEnumTerminal>;
    fn Terminals(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Stream(&mut self) -> ::windows::core::Result<ITStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITSubStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSubStreamVtbl {
        unsafe extern "system" fn StartSubStream<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartSubStream().into()
        }
        unsafe extern "system" fn PauseSubStream<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseSubStream().into()
        }
        unsafe extern "system" fn StopSubStream<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopSubStream().into()
        }
        unsafe extern "system" fn SelectTerminal<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnselectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: ITSubStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartSubStream: StartSubStream::<Impl, IMPL_OFFSET>,
            PauseSubStream: PauseSubStream::<Impl, IMPL_OFFSET>,
            StopSubStream: StopSubStream::<Impl, IMPL_OFFSET>,
            SelectTerminal: SelectTerminal::<Impl, IMPL_OFFSET>,
            UnselectTerminal: UnselectTerminal::<Impl, IMPL_OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Impl, IMPL_OFFSET>,
            Terminals: Terminals::<Impl, IMPL_OFFSET>,
            Stream: Stream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSubStream as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITSubStreamControlImpl: Sized + IDispatchImpl {
    fn CreateSubStream(&mut self) -> ::windows::core::Result<ITSubStream>;
    fn RemoveSubStream(&mut self, psubstream: ::core::option::Option<ITSubStream>) -> ::windows::core::Result<()>;
    fn EnumerateSubStreams(&mut self) -> ::windows::core::Result<IEnumSubStream>;
    fn SubStreams(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITSubStreamControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSubStreamControlVtbl {
        unsafe extern "system" fn CreateSubStream<Impl: ITSubStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSubStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubStream<Impl: ITSubStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubStream(::core::mem::transmute(&psubstream)).into()
        }
        unsafe extern "system" fn EnumerateSubStreams<Impl: ITSubStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSubStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsubstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubStreams<Impl: ITSubStreamControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateSubStream: CreateSubStream::<Impl, IMPL_OFFSET>,
            RemoveSubStream: RemoveSubStream::<Impl, IMPL_OFFSET>,
            EnumerateSubStreams: EnumerateSubStreams::<Impl, IMPL_OFFSET>,
            SubStreams: SubStreams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSubStreamControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIImpl: Sized + IDispatchImpl {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
    fn Addresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateAddresses(&mut self) -> ::windows::core::Result<IEnumAddress>;
    fn RegisterCallNotifications(&mut self, paddress: ::core::option::Option<ITAddress>, fmonitor: i16, fowner: i16, lmediatypes: i32, lcallbackinstance: i32) -> ::windows::core::Result<i32>;
    fn UnregisterNotifications(&mut self, lregister: i32) -> ::windows::core::Result<()>;
    fn CallHubs(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCallHubs(&mut self) -> ::windows::core::Result<IEnumCallHub>;
    fn SetCallHubTracking(&mut self, paddresses: super::super::System::Com::VARIANT, btracking: i16) -> ::windows::core::Result<()>;
    fn EnumeratePrivateTAPIObjects(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
    fn PrivateTAPIObjects(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RegisterRequestRecipient(&mut self, lregistrationinstance: i32, lrequestmode: i32, fenable: i16) -> ::windows::core::Result<()>;
    fn SetAssistedTelephonyPriority(&mut self, pappfilename: super::super::Foundation::BSTR, fpriority: i16) -> ::windows::core::Result<()>;
    fn SetApplicationPriority(&mut self, pappfilename: super::super::Foundation::BSTR, lmediatype: i32, fpriority: i16) -> ::windows::core::Result<()>;
    fn SetEventFilter(&mut self, lfiltermask: i32) -> ::windows::core::Result<()>;
    fn EventFilter(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIVtbl {
        unsafe extern "system" fn Initialize<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn Shutdown<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn Addresses<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallNotifications<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, fmonitor: i16, fowner: i16, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterCallNotifications(::core::mem::transmute(&paddress), ::core::mem::transmute_copy(&fmonitor), ::core::mem::transmute_copy(&fowner), ::core::mem::transmute_copy(&lmediatypes), ::core::mem::transmute_copy(&lcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *plregister = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNotifications<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lregister: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterNotifications(::core::mem::transmute_copy(&lregister)).into()
        }
        unsafe extern "system" fn CallHubs<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHubs() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallHubs<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCallHubs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHubTracking<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, btracking: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallHubTracking(::core::mem::transmute_copy(&paddresses), ::core::mem::transmute_copy(&btracking)).into()
        }
        unsafe extern "system" fn EnumeratePrivateTAPIObjects<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePrivateTAPIObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateTAPIObjects<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateTAPIObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRequestRecipient<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lregistrationinstance: i32, lrequestmode: i32, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterRequestRecipient(::core::mem::transmute_copy(&lregistrationinstance), ::core::mem::transmute_copy(&lrequestmode), ::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SetAssistedTelephonyPriority<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAssistedTelephonyPriority(::core::mem::transmute_copy(&pappfilename), ::core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetApplicationPriority<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationPriority(::core::mem::transmute_copy(&pappfilename), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfiltermask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&lfiltermask)).into()
        }
        unsafe extern "system" fn EventFilter<Impl: ITTAPIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfiltermask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *plfiltermask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
            Addresses: Addresses::<Impl, IMPL_OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Impl, IMPL_OFFSET>,
            RegisterCallNotifications: RegisterCallNotifications::<Impl, IMPL_OFFSET>,
            UnregisterNotifications: UnregisterNotifications::<Impl, IMPL_OFFSET>,
            CallHubs: CallHubs::<Impl, IMPL_OFFSET>,
            EnumerateCallHubs: EnumerateCallHubs::<Impl, IMPL_OFFSET>,
            SetCallHubTracking: SetCallHubTracking::<Impl, IMPL_OFFSET>,
            EnumeratePrivateTAPIObjects: EnumeratePrivateTAPIObjects::<Impl, IMPL_OFFSET>,
            PrivateTAPIObjects: PrivateTAPIObjects::<Impl, IMPL_OFFSET>,
            RegisterRequestRecipient: RegisterRequestRecipient::<Impl, IMPL_OFFSET>,
            SetAssistedTelephonyPriority: SetAssistedTelephonyPriority::<Impl, IMPL_OFFSET>,
            SetApplicationPriority: SetApplicationPriority::<Impl, IMPL_OFFSET>,
            SetEventFilter: SetEventFilter::<Impl, IMPL_OFFSET>,
            EventFilter: EventFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPI2Impl: Sized + IDispatchImpl + ITTAPIImpl {
    fn Phones(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePhones(&mut self) -> ::windows::core::Result<IEnumPhone>;
    fn CreateEmptyCollectionObject(&mut self) -> ::windows::core::Result<ITCollection2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPI2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPI2Vtbl {
        unsafe extern "system" fn Phones<Impl: ITTAPI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phones() {
                ::core::result::Result::Ok(ok__) => {
                    *pphones = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Impl: ITTAPI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePhones() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEmptyCollectionObject<Impl: ITTAPI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEmptyCollectionObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITTAPIVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Phones: Phones::<Impl, IMPL_OFFSET>,
            EnumeratePhones: EnumeratePhones::<Impl, IMPL_OFFSET>,
            CreateEmptyCollectionObject: CreateEmptyCollectionObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPI2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPICallCenterImpl: Sized + IDispatchImpl {
    fn EnumerateAgentHandlers(&mut self) -> ::windows::core::Result<IEnumAgentHandler>;
    fn AgentHandlers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPICallCenterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPICallCenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPICallCenterVtbl {
        unsafe extern "system" fn EnumerateAgentHandlers<Impl: ITTAPICallCenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAgentHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumhandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentHandlers<Impl: ITTAPICallCenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgentHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumerateAgentHandlers: EnumerateAgentHandlers::<Impl, IMPL_OFFSET>,
            AgentHandlers: AgentHandlers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPICallCenter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIDispatchEventNotificationImpl: Sized + IDispatchImpl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIDispatchEventNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIDispatchEventNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIDispatchEventNotificationVtbl {
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIDispatchEventNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIEventNotificationImpl: Sized {
    fn Event(&mut self, tapievent: TAPI_EVENT, pevent: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIEventNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIEventNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIEventNotificationVtbl {
        unsafe extern "system" fn Event<Impl: ITTAPIEventNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Event(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute(&pevent)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Event: Event::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIEventNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIObjectEventImpl: Sized + IDispatchImpl {
    fn TAPIObject(&mut self) -> ::windows::core::Result<ITTAPI>;
    fn Event(&mut self) -> ::windows::core::Result<TAPIOBJECT_EVENT>;
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIObjectEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIObjectEventVtbl {
        unsafe extern "system" fn TAPIObject<Impl: ITTAPIObjectEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TAPIObject() {
                ::core::result::Result::Ok(ok__) => {
                    *pptapiobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITTAPIObjectEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut TAPIOBJECT_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: ITTAPIObjectEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITTAPIObjectEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TAPIObject: TAPIObject::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIObjectEvent2Impl: Sized + IDispatchImpl + ITTAPIObjectEventImpl {
    fn Phone(&mut self) -> ::windows::core::Result<ITPhone>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIObjectEvent2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIObjectEvent2Vtbl {
        unsafe extern "system" fn Phone<Impl: ITTAPIObjectEvent2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITTAPIObjectEventVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Phone: Phone::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTTSTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTTSTerminalEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTTSTerminalEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTTSTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITTTSTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITTTSTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITTTSTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTTSTerminalEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&mut self) -> ::windows::core::Result<TERMINAL_STATE>;
    fn TerminalType(&mut self) -> ::windows::core::Result<TERMINAL_TYPE>;
    fn TerminalClass(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MediaType(&mut self) -> ::windows::core::Result<i32>;
    fn Direction(&mut self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTerminalVtbl {
        unsafe extern "system" fn Name<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalstate: *mut TERMINAL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminalstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalType<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TERMINAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalClass() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminalclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ITTerminalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            TerminalType: TerminalType::<Impl, IMPL_OFFSET>,
            TerminalClass: TerminalClass::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalSupportImpl: Sized + IDispatchImpl {
    fn StaticTerminals(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateStaticTerminals(&mut self) -> ::windows::core::Result<IEnumTerminal>;
    fn DynamicTerminalClasses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDynamicTerminalClasses(&mut self) -> ::windows::core::Result<IEnumTerminalClass>;
    fn CreateTerminal(&mut self, pterminalclass: super::super::Foundation::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
    fn GetDefaultStaticTerminal(&mut self, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTerminalSupportVtbl {
        unsafe extern "system" fn StaticTerminals<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaticTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateStaticTerminals<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateStaticTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminalenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicTerminalClasses<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DynamicTerminalClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDynamicTerminalClasses<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateDynamicTerminalClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminalclassenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTerminal<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTerminal(::core::mem::transmute_copy(&pterminalclass), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultStaticTerminal<Impl: ITTerminalSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultStaticTerminal(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StaticTerminals: StaticTerminals::<Impl, IMPL_OFFSET>,
            EnumerateStaticTerminals: EnumerateStaticTerminals::<Impl, IMPL_OFFSET>,
            DynamicTerminalClasses: DynamicTerminalClasses::<Impl, IMPL_OFFSET>,
            EnumerateDynamicTerminalClasses: EnumerateDynamicTerminalClasses::<Impl, IMPL_OFFSET>,
            CreateTerminal: CreateTerminal::<Impl, IMPL_OFFSET>,
            GetDefaultStaticTerminal: GetDefaultStaticTerminal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminalSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalSupport2Impl: Sized + IDispatchImpl + ITTerminalSupportImpl {
    fn PluggableSuperclasses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePluggableSuperclasses(&mut self) -> ::windows::core::Result<IEnumPluggableSuperclassInfo>;
    fn PluggableTerminalClasses(&mut self, bstrterminalsuperclass: super::super::Foundation::BSTR, lmediatype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePluggableTerminalClasses(&mut self, iidterminalsuperclass: ::windows::core::GUID, lmediatype: i32) -> ::windows::core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalSupport2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTerminalSupport2Vtbl {
        unsafe extern "system" fn PluggableSuperclasses<Impl: ITTerminalSupport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PluggableSuperclasses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableSuperclasses<Impl: ITTerminalSupport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuperclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePluggableSuperclasses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsuperclassenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PluggableTerminalClasses<Impl: ITTerminalSupport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrterminalsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PluggableTerminalClasses(::core::mem::transmute_copy(&bstrterminalsuperclass), ::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableTerminalClasses<Impl: ITTerminalSupport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iidterminalsuperclass: ::windows::core::GUID, lmediatype: i32, ppclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePluggableTerminalClasses(::core::mem::transmute_copy(&iidterminalsuperclass), ::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclassenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITTerminalSupportVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PluggableSuperclasses: PluggableSuperclasses::<Impl, IMPL_OFFSET>,
            EnumeratePluggableSuperclasses: EnumeratePluggableSuperclasses::<Impl, IMPL_OFFSET>,
            PluggableTerminalClasses: PluggableTerminalClasses::<Impl, IMPL_OFFSET>,
            EnumeratePluggableTerminalClasses: EnumeratePluggableTerminalClasses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminalSupport2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITToneDetectionEventImpl: Sized + IDispatchImpl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn TickCount(&mut self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITToneDetectionEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITToneDetectionEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITToneDetectionEventVtbl {
        unsafe extern "system" fn Call<Impl: ITToneDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Impl: ITToneDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITToneDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITToneDetectionEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            AppSpecific: AppSpecific::<Impl, IMPL_OFFSET>,
            TickCount: TickCount::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITToneDetectionEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITToneTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITToneTerminalEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITToneTerminalEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITToneTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITToneTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITToneTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITToneTerminalEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITToneTerminalEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub trait ITnefImpl: Sized {
    fn AddProps(&mut self, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::Result<()>;
    fn ExtractProps(&mut self, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()>;
    fn Finish(&mut self, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()>;
    fn OpenTaggedBody(&mut self, lpmessage: ::core::option::Option<super::super::System::AddressBook::IMessage>, ulflags: u32) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetProps(&mut self, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::Result<()>;
    fn EncodeRecips(&mut self, ulflags: u32, lprecipienttable: ::core::option::Option<super::super::System::AddressBook::IMAPITable>) -> ::windows::core::Result<()>;
    fn FinishComponent(&mut self, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
impl ITnefVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITnefImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITnefVtbl {
        unsafe extern "system" fn AddProps<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddProps(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulelemid), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&lpproplist)).into()
        }
        unsafe extern "system" fn ExtractProps<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExtractProps(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpproplist), ::core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn Finish<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpkey), ::core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn OpenTaggedBody<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmessage: ::windows::core::RawPtr, ulflags: u32, lppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTaggedBody(::core::mem::transmute(&lpmessage), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProps<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProps(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulelemid), ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpprops)).into()
        }
        unsafe extern "system" fn EncodeRecips<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lprecipienttable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncodeRecips(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute(&lprecipienttable)).into()
        }
        unsafe extern "system" fn FinishComponent<Impl: ITnefImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinishComponent(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulcomponentid), ::core::mem::transmute_copy(&lpcustomproplist), ::core::mem::transmute_copy(&lpcustomprops), ::core::mem::transmute_copy(&lpproplist), ::core::mem::transmute_copy(&lpproblems)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddProps: AddProps::<Impl, IMPL_OFFSET>,
            ExtractProps: ExtractProps::<Impl, IMPL_OFFSET>,
            Finish: Finish::<Impl, IMPL_OFFSET>,
            OpenTaggedBody: OpenTaggedBody::<Impl, IMPL_OFFSET>,
            SetProps: SetProps::<Impl, IMPL_OFFSET>,
            EncodeRecips: EncodeRecips::<Impl, IMPL_OFFSET>,
            FinishComponent: FinishComponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITnef as ::windows::core::Interface>::IID
    }
}
