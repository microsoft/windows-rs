#[cfg(feature = "Win32_System_Com")]
pub trait IEnumACDGroup_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITACDGroup>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumACDGroup>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumACDGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumACDGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumACDGroup_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumAddress_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITAddress>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumAddress>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAddress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAddress_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumAgent_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITAgent>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumAgent>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAgent_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumAgentHandler_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITAgentHandler>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumAgentHandler>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAgentHandler_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumAgentSession_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITAgentSession>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumAgentSession>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgentSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumAgentSession_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumBstr_Impl: Sized {
    fn Next(&mut self, celt: u32, ppstrings: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumBstr>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumBstr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBstr_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumBstr_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstrings: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstrings), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumCall_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITCallInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCall>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCall_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCall_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCall_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumCallHub_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITCallHub>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCallHub>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCallHub_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallHub_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCallHub_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumCallingCard_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITCallingCard>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumCallingCard>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCallingCard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallingCard_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCallingCard_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumDialableAddrs_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDialableAddrs>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumDialableAddrs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDialableAddrs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDialableAddrs_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumDirectory_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITDirectory>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDirectory>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumDirectory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDirectory_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumDirectoryObject_Impl: Sized {
    fn Next(&mut self, celt: u32, pval: *mut ::core::option::Option<ITDirectoryObject>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDirectoryObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumDirectoryObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDirectoryObject_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pval: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumLocation_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITLocationInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumLocation>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumLocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumLocation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumLocation_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumMcastScope_Impl: Sized {
    fn Next(&mut self, celt: u32, ppscopes: *mut ::core::option::Option<IMcastScope>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumMcastScope>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMcastScope_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMcastScope_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumMcastScope_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppscopes: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppscopes), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumPhone_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITPhone>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumPhone>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPhone_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPhone_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPhone_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumPluggableSuperclassInfo_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITPluggableTerminalSuperclassInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumPluggableSuperclassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPluggableSuperclassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableSuperclassInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPluggableSuperclassInfo_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumPluggableTerminalClassInfo_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITPluggableTerminalClassInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPluggableTerminalClassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableTerminalClassInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumPluggableTerminalClassInfo_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumQueue_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITQueue>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumQueue>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumQueue_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumStream_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITStream>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumStream_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumSubStream_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITSubStream>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumSubStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumSubStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSubStream_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumTerminal_Impl: Sized {
    fn Next(&mut self, celt: u32, ppelements: *mut ::core::option::Option<ITTerminal>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTerminal>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTerminal_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumTerminalClass_Impl: Sized {
    fn Next(&mut self, celt: u32, pelements: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumTerminalClass>;
}
impl IEnumTerminalClass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalClass_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumTerminalClass_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pelements: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IMcastAddressAllocation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Scopes(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateScopes(&mut self) -> ::windows::core::Result<IEnumMcastScope>;
    fn RequestAddress(&mut self, pscope: &::core::option::Option<IMcastScope>, leasestarttime: f64, leasestoptime: f64, numaddresses: i32) -> ::windows::core::Result<IMcastLeaseInfo>;
    fn RenewAddress(&mut self, lreserved: i32, prenewrequest: &::core::option::Option<IMcastLeaseInfo>) -> ::windows::core::Result<IMcastLeaseInfo>;
    fn ReleaseAddress(&mut self, preleaserequest: &::core::option::Option<IMcastLeaseInfo>) -> ::windows::core::Result<()>;
    fn CreateLeaseInfo(&mut self, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const super::super::Foundation::PWSTR, prequestid: super::super::Foundation::PWSTR, pserveraddress: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMcastLeaseInfo>;
    fn CreateLeaseInfoFromVariant(&mut self, leasestarttime: f64, leasestoptime: f64, vaddresses: &super::super::System::Com::VARIANT, prequestid: &super::super::Foundation::BSTR, pserveraddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<IMcastLeaseInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastAddressAllocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMcastAddressAllocation_Vtbl {
        unsafe extern "system" fn Scopes<Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scopes() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateScopes<Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenummcastscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateScopes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenummcastscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAddress<Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscope: ::windows::core::RawPtr, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAddress(::core::mem::transmute(&pscope), ::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute_copy(&numaddresses)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppleaseresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewAddress<Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lreserved: i32, prenewrequest: ::windows::core::RawPtr, pprenewresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewAddress(::core::mem::transmute_copy(&lreserved), ::core::mem::transmute(&prenewrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprenewresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseAddress<Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preleaserequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseAddress(::core::mem::transmute(&preleaserequest)).into()
        }
        unsafe extern "system" fn CreateLeaseInfo<Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const super::super::Foundation::PWSTR, prequestid: super::super::Foundation::PWSTR, pserveraddress: super::super::Foundation::PWSTR, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLeaseInfo(::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute_copy(&dwnumaddresses), ::core::mem::transmute_copy(&ppaddresses), ::core::mem::transmute_copy(&prequestid), ::core::mem::transmute_copy(&pserveraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreleaserequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLeaseInfoFromVariant<Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, vaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, prequestid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pserveraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<IMcastAddressAllocation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastLeaseInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
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
impl IMcastLeaseInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMcastLeaseInfo_Vtbl {
        unsafe extern "system" fn RequestID<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprequestid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestID() {
                ::core::result::Result::Ok(ok__) => {
                    *pprequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseStartTime<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeaseStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStartTime<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeaseStartTime(::core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn LeaseStopTime<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeaseStopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStopTime<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLeaseStopTime(::core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn AddressCount<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerAddress<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Addresses<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddresses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<IMcastLeaseInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastScope_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ScopeID(&mut self) -> ::windows::core::Result<i32>;
    fn ServerID(&mut self) -> ::windows::core::Result<i32>;
    fn InterfaceID(&mut self) -> ::windows::core::Result<i32>;
    fn ScopeDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TTL(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastScope_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastScope_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMcastScope_Vtbl {
        unsafe extern "system" fn ScopeID<Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerID<Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeDescription<Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ScopeID: ScopeID::<Impl, IMPL_OFFSET>,
            ServerID: ServerID::<Impl, IMPL_OFFSET>,
            InterfaceID: InterfaceID::<Impl, IMPL_OFFSET>,
            ScopeDescription: ScopeDescription::<Impl, IMPL_OFFSET>,
            TTL: TTL::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastScope as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITACDGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnumerateQueues(&mut self) -> ::windows::core::Result<IEnumQueue>;
    fn Queues(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITACDGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITACDGroup_Vtbl {
        unsafe extern "system" fn Name<Impl: ITACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateQueues<Impl: ITACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Queues<Impl: ITACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            EnumerateQueues: EnumerateQueues::<Impl, IMPL_OFFSET>,
            Queues: Queues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITACDGroup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITACDGroupEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Group(&mut self) -> ::windows::core::Result<ITACDGroup>;
    fn Event(&mut self) -> ::windows::core::Result<ACDGROUP_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITACDGroupEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroupEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITACDGroupEvent_Vtbl {
        unsafe extern "system" fn Group<Impl: ITACDGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITACDGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDGROUP_EVENT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Group: Group::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITACDGroupEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait ITAMMediaFormat_Impl: Sized {
    fn MediaFormat(&mut self) -> ::windows::core::Result<*mut super::super::Media::DirectShow::AM_MEDIA_TYPE>;
    fn SetMediaFormat(&mut self, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ITAMMediaFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAMMediaFormat_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAMMediaFormat_Vtbl {
        unsafe extern "system" fn MediaFormat<Impl: ITAMMediaFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaFormat<Impl: ITAMMediaFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
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
pub trait ITASRTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITASRTerminalEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITASRTerminalEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITASRTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITASRTerminalEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn State(&mut self) -> ::windows::core::Result<ADDRESS_STATE>;
    fn AddressName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceProviderName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TAPIObject(&mut self) -> ::windows::core::Result<ITTAPI>;
    fn CreateCall(&mut self, pdestaddress: &super::super::Foundation::BSTR, laddresstype: i32, lmediatypes: i32) -> ::windows::core::Result<ITBasicCallControl>;
    fn Calls(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCalls(&mut self) -> ::windows::core::Result<IEnumCall>;
    fn DialableAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateForwardInfoObject(&mut self) -> ::windows::core::Result<ITForwardInformation>;
    fn Forward(&mut self, pforwardinfo: &::core::option::Option<ITForwardInformation>, pcall: &::core::option::Option<ITBasicCallControl>) -> ::windows::core::Result<()>;
    fn CurrentForwardInfo(&mut self) -> ::windows::core::Result<ITForwardInformation>;
    fn SetMessageWaiting(&mut self, fmessagewaiting: i16) -> ::windows::core::Result<()>;
    fn MessageWaiting(&mut self) -> ::windows::core::Result<i16>;
    fn SetDoNotDisturb(&mut self, fdonotdisturb: i16) -> ::windows::core::Result<()>;
    fn DoNotDisturb(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddress_Vtbl {
        unsafe extern "system" fn State<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddressstate: *mut ADDRESS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *paddressstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressName<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProviderName<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TAPIObject<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TAPIObject() {
                ::core::result::Result::Ok(ok__) => {
                    *pptapiobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCall<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, laddresstype: i32, lmediatypes: i32, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCall(::core::mem::transmute_copy(&pdestaddress), ::core::mem::transmute_copy(&laddresstype), ::core::mem::transmute_copy(&lmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Calls() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCalls<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialableAddress<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdialableaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialableAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pdialableaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForwardInfoObject<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForwardInfoObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppforwardinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forward<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pforwardinfo: ::windows::core::RawPtr, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Forward(::core::mem::transmute(&pforwardinfo), ::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn CurrentForwardInfo<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentForwardInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppforwardinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageWaiting<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmessagewaiting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageWaiting(::core::mem::transmute_copy(&fmessagewaiting)).into()
        }
        unsafe extern "system" fn MessageWaiting<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmessagewaiting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageWaiting() {
                ::core::result::Result::Ok(ok__) => {
                    *pfmessagewaiting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotDisturb<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdonotdisturb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDoNotDisturb(::core::mem::transmute_copy(&fdonotdisturb)).into()
        }
        unsafe extern "system" fn DoNotDisturb<Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdonotdisturb: *mut i16) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITAddress as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddress2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITAddress_Impl {
    fn Phones(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePhones(&mut self) -> ::windows::core::Result<IEnumPhone>;
    fn GetPhoneFromTerminal(&mut self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<ITPhone>;
    fn PreferredPhones(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePreferredPhones(&mut self) -> ::windows::core::Result<IEnumPhone>;
    fn EventFilter(&mut self, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows::core::Result<i16>;
    fn SetEventFilter(&mut self, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::Result<()>;
    fn DeviceSpecific(&mut self, pcall: &::core::option::Option<ITCallInfo>, pparams: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn DeviceSpecificVariant(&mut self, pcall: &::core::option::Option<ITCallInfo>, vardevspecificbytearray: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NegotiateExtVersion(&mut self, llowversion: i32, lhighversion: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddress2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddress2_Vtbl {
        unsafe extern "system" fn Phones<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phones() {
                ::core::result::Result::Ok(ok__) => {
                    *pphones = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePhones() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneFromTerminal<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPhoneFromTerminal(::core::mem::transmute(&pterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPhones<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredPhones() {
                ::core::result::Result::Ok(ok__) => {
                    *pphones = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredPhones<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePreferredPhones() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventFilter<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *penable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn DeviceSpecific<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSpecific(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSpecificVariant(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
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
            base: ITAddress_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITAddress2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressCapabilities_Impl: Sized + super::super::System::Com::IDispatch_Impl {
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
impl ITAddressCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressCapabilities_Vtbl {
        unsafe extern "system" fn AddressCapability<Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressCapability(::core::mem::transmute_copy(&addresscap)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressCapabilityString<Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddressCapabilityString(::core::mem::transmute_copy(&addresscapstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilitystring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallTreatments<Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallTreatments() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallTreatments<Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcalltreatment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCallTreatments() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcalltreatment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompletionMessages<Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompletionMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCompletionMessages<Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcompletionmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCompletionMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcompletionmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceClasses<Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDeviceClasses<Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdeviceclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITAddressCapabilities as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressDeviceSpecificEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn lParam1(&mut self) -> ::windows::core::Result<i32>;
    fn lParam2(&mut self) -> ::windows::core::Result<i32>;
    fn lParam3(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressDeviceSpecificEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressDeviceSpecificEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressDeviceSpecificEvent_Vtbl {
        unsafe extern "system" fn Address<Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam1() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam2() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Address: Address::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            lParam1: lParam1::<Impl, IMPL_OFFSET>,
            lParam2: lParam2::<Impl, IMPL_OFFSET>,
            lParam3: lParam3::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressDeviceSpecificEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn Event(&mut self) -> ::windows::core::Result<ADDRESS_EVENT>;
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressEvent_Vtbl {
        unsafe extern "system" fn Address<Impl: ITAddressEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAddressEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ADDRESS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Impl: ITAddressEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Address: Address::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressTranslation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TranslateAddress(&mut self, paddresstotranslate: &super::super::Foundation::BSTR, lcard: i32, ltranslateoptions: i32) -> ::windows::core::Result<ITAddressTranslationInfo>;
    fn TranslateDialog(&mut self, hwndowner: isize, paddressin: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EnumerateLocations(&mut self) -> ::windows::core::Result<IEnumLocation>;
    fn Locations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCallingCards(&mut self) -> ::windows::core::Result<IEnumCallingCard>;
    fn CallingCards(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressTranslation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressTranslation_Vtbl {
        unsafe extern "system" fn TranslateAddress<Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresstotranslate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcard: i32, ltranslateoptions: i32, pptranslated: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslateAddress(::core::mem::transmute_copy(&paddresstotranslate), ::core::mem::transmute_copy(&lcard), ::core::mem::transmute_copy(&ltranslateoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptranslated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateDialog<Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: isize, paddressin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TranslateDialog(::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&paddressin)).into()
        }
        unsafe extern "system" fn EnumerateLocations<Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumlocation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumlocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locations<Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Locations() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallingCards<Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcallingcard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCallingCards() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcallingcard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallingCards<Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TranslateAddress: TranslateAddress::<Impl, IMPL_OFFSET>,
            TranslateDialog: TranslateDialog::<Impl, IMPL_OFFSET>,
            EnumerateLocations: EnumerateLocations::<Impl, IMPL_OFFSET>,
            Locations: Locations::<Impl, IMPL_OFFSET>,
            EnumerateCallingCards: EnumerateCallingCards::<Impl, IMPL_OFFSET>,
            CallingCards: CallingCards::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressTranslation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressTranslationInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DialableString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayableString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentCountryCode(&mut self) -> ::windows::core::Result<i32>;
    fn DestinationCountryCode(&mut self) -> ::windows::core::Result<i32>;
    fn TranslationResults(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressTranslationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAddressTranslationInfo_Vtbl {
        unsafe extern "system" fn DialableString<Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdialablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialableString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdialablestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayableString<Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplayablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayableString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisplayablestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCountryCode<Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *countrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationCountryCode<Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestinationCountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *countrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslationResults<Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plresults: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DialableString: DialableString::<Impl, IMPL_OFFSET>,
            DisplayableString: DisplayableString::<Impl, IMPL_OFFSET>,
            CurrentCountryCode: CurrentCountryCode::<Impl, IMPL_OFFSET>,
            DestinationCountryCode: DestinationCountryCode::<Impl, IMPL_OFFSET>,
            TranslationResults: TranslationResults::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressTranslationInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumerateAgentSessions(&mut self) -> ::windows::core::Result<IEnumAgentSession>;
    fn CreateSession(&mut self, pacdgroup: &::core::option::Option<ITACDGroup>, paddress: &::core::option::Option<ITAddress>) -> ::windows::core::Result<ITAgentSession>;
    fn CreateSessionWithPIN(&mut self, pacdgroup: &::core::option::Option<ITACDGroup>, paddress: &::core::option::Option<ITAddress>, ppin: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITAgentSession>;
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
impl ITAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgent_Vtbl {
        unsafe extern "system" fn EnumerateAgentSessions<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAgentSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumagentsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSession(::core::mem::transmute(&pacdgroup), ::core::mem::transmute(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppagentsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionWithPIN<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSessionWithPIN(::core::mem::transmute(&pacdgroup), ::core::mem::transmute(&paddress), ::core::mem::transmute_copy(&ppin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppagentsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *ppuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agentstate: AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&agentstate)).into()
        }
        unsafe extern "system" fn State<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagentstate: *mut AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pagentstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeasurementPeriod<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMeasurementPeriod(::core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasurementPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *plperiod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallCallRate<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverallCallRate() {
                ::core::result::Result::Ok(ok__) => {
                    *pcycallrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfACDCalls<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfACDCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfIncomingCalls<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfIncomingCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfOutgoingCalls<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfOutgoingCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDTalkTime<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalACDTalkTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pltalktime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDCallTime<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalACDCallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalWrapUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwrapuptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentSessions<Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITAgent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Agent(&mut self) -> ::windows::core::Result<ITAgent>;
    fn Event(&mut self) -> ::windows::core::Result<AGENT_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentEvent_Vtbl {
        unsafe extern "system" fn Agent<Impl: ITAgentEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Agent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAgentEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_EVENT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Agent: Agent::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentHandler_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateAgent(&mut self) -> ::windows::core::Result<ITAgent>;
    fn CreateAgentWithID(&mut self, pid: &super::super::Foundation::BSTR, ppin: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITAgent>;
    fn EnumerateACDGroups(&mut self) -> ::windows::core::Result<IEnumACDGroup>;
    fn EnumerateUsableAddresses(&mut self) -> ::windows::core::Result<IEnumAddress>;
    fn ACDGroups(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn UsableAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentHandler_Vtbl {
        unsafe extern "system" fn Name<Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgent<Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAgent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgentWithID<Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAgentWithID(::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&ppin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateACDGroups<Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateACDGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumacdgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateUsableAddresses<Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateUsableAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroups<Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ACDGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsableAddresses<Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITAgentHandler as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentHandlerEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AgentHandler(&mut self) -> ::windows::core::Result<ITAgentHandler>;
    fn Event(&mut self) -> ::windows::core::Result<AGENTHANDLER_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentHandlerEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandlerEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentHandlerEvent_Vtbl {
        unsafe extern "system" fn AgentHandler<Impl: ITAgentHandlerEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagenthandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagenthandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAgentHandlerEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENTHANDLER_EVENT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AgentHandler: AgentHandler::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentHandlerEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentSession_Impl: Sized + super::super::System::Com::IDispatch_Impl {
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
impl ITAgentSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentSession_Vtbl {
        unsafe extern "system" fn Agent<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Agent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroup<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ACDGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *ppacdgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionstate: AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&sessionstate)).into()
        }
        unsafe extern "system" fn State<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: *mut AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *psessionstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartTime<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesessionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatesessionstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionDuration<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *plduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfCalls<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTalkTime<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalTalkTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pltalktime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTalkTime<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AverageTalkTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pltalktime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallTime<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageCallTime<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AverageCallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalWrapUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwrapuptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWrapUpTime<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AverageWrapUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwrapuptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDCallRate<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ACDCallRate() {
                ::core::result::Result::Ok(ok__) => {
                    *pcycallrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestTimeToAnswer<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongestTimeToAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    *planswertime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTimeToAnswer<Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITAgentSession as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentSessionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Session(&mut self) -> ::windows::core::Result<ITAgentSession>;
    fn Event(&mut self) -> ::windows::core::Result<AGENT_SESSION_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentSessionEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSessionEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAgentSessionEvent_Vtbl {
        unsafe extern "system" fn Session<Impl: ITAgentSessionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAgentSessionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_SESSION_EVENT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentSessionEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait ITAllocatorProperties_Impl: Sized {
    fn SetAllocatorProperties(&mut self, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::Result<()>;
    fn GetAllocatorProperties(&mut self) -> ::windows::core::Result<super::super::Media::DirectShow::ALLOCATOR_PROPERTIES>;
    fn SetAllocateBuffers(&mut self, ballocbuffers: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateBuffers(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBufferSize(&mut self, buffersize: u32) -> ::windows::core::Result<()>;
    fn GetBufferSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ITAllocatorProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAllocatorProperties_Vtbl {
        unsafe extern "system" fn SetAllocatorProperties<Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllocatorProperties(::core::mem::transmute_copy(&pallocproperties)).into()
        }
        unsafe extern "system" fn GetAllocatorProperties<Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllocatorProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pallocproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateBuffers<Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballocbuffers: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllocateBuffers(::core::mem::transmute_copy(&ballocbuffers)).into()
        }
        unsafe extern "system" fn GetAllocateBuffers<Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballocbuffers: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllocateBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *pballocbuffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferSize(::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn GetBufferSize<Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffersize: *mut u32) -> ::windows::core::HRESULT {
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
pub trait ITAutomatedPhoneControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
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
    fn SelectCall(&mut self, pcall: &::core::option::Option<ITCallInfo>, fselectdefaultterminals: i16) -> ::windows::core::Result<()>;
    fn UnselectCall(&mut self, pcall: &::core::option::Option<ITCallInfo>) -> ::windows::core::Result<()>;
    fn EnumerateSelectedCalls(&mut self) -> ::windows::core::Result<IEnumCall>;
    fn SelectedCalls(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAutomatedPhoneControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITAutomatedPhoneControl_Vtbl {
        unsafe extern "system" fn StartTone<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tone: PHONE_TONE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTone(::core::mem::transmute_copy(&tone), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopTone<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopTone().into()
        }
        unsafe extern "system" fn Tone<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptone: *mut PHONE_TONE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tone() {
                ::core::result::Result::Ok(ok__) => {
                    *ptone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRinger<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartRinger(::core::mem::transmute_copy(&lringmode), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopRinger<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopRinger().into()
        }
        unsafe extern "system" fn Ringer<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfringing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ringer() {
                ::core::result::Result::Ok(ok__) => {
                    *pfringing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhoneHandlingEnabled<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPhoneHandlingEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn PhoneHandlingEnabled<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneHandlingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoEndOfNumberTimeout<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoEndOfNumberTimeout(::core::mem::transmute_copy(&ltimeout)).into()
        }
        unsafe extern "system" fn AutoEndOfNumberTimeout<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltimeout: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoEndOfNumberTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *pltimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDialtone<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoDialtone(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoDialtone<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoDialtone() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopTonesOnOnHook<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoStopTonesOnOnHook(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopTonesOnOnHook<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoStopTonesOnOnHook() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopRingOnOffHook<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoStopRingOnOffHook(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopRingOnOffHook<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoStopRingOnOffHook() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTones<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoKeypadTones(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoKeypadTones<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoKeypadTones() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTonesMinimumDuration<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoKeypadTonesMinimumDuration(::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn AutoKeypadTonesMinimumDuration<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoKeypadTonesMinimumDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *plduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControl<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoVolumeControl(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoVolumeControl<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControl() {
                ::core::result::Result::Ok(ok__) => {
                    *fenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlStep<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstepsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoVolumeControlStep(::core::mem::transmute_copy(&lstepsize)).into()
        }
        unsafe extern "system" fn AutoVolumeControlStep<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstepsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControlStep() {
                ::core::result::Result::Ok(ok__) => {
                    *plstepsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatDelay<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoVolumeControlRepeatDelay(::core::mem::transmute_copy(&ldelay)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatDelay<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControlRepeatDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatPeriod<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoVolumeControlRepeatPeriod(::core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatPeriod<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControlRepeatPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *plperiod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectCall<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fselectdefaultterminals: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectCall(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&fselectdefaultterminals)).into()
        }
        unsafe extern "system" fn UnselectCall<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnselectCall(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn EnumerateSelectedCalls<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSelectedCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedCalls<Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITAutomatedPhoneControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicAudioTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetVolume(&mut self, lvolume: i32) -> ::windows::core::Result<()>;
    fn Volume(&mut self) -> ::windows::core::Result<i32>;
    fn SetBalance(&mut self, lbalance: i32) -> ::windows::core::Result<()>;
    fn Balance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicAudioTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicAudioTerminal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITBasicAudioTerminal_Vtbl {
        unsafe extern "system" fn SetVolume<Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn Volume<Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBalance<Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBalance(::core::mem::transmute_copy(&lbalance)).into()
        }
        unsafe extern "system" fn Balance<Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            Volume: Volume::<Impl, IMPL_OFFSET>,
            SetBalance: SetBalance::<Impl, IMPL_OFFSET>,
            Balance: Balance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicAudioTerminal as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicCallControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Connect(&mut self, fsync: i16) -> ::windows::core::Result<()>;
    fn Answer(&mut self) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self, code: DISCONNECT_CODE) -> ::windows::core::Result<()>;
    fn Hold(&mut self, fhold: i16) -> ::windows::core::Result<()>;
    fn HandoffDirect(&mut self, papplicationname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HandoffIndirect(&mut self, lmediatype: i32) -> ::windows::core::Result<()>;
    fn Conference(&mut self, pcall: &::core::option::Option<ITBasicCallControl>, fsync: i16) -> ::windows::core::Result<()>;
    fn Transfer(&mut self, pcall: &::core::option::Option<ITBasicCallControl>, fsync: i16) -> ::windows::core::Result<()>;
    fn BlindTransfer(&mut self, pdestaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SwapHold(&mut self, pcall: &::core::option::Option<ITBasicCallControl>) -> ::windows::core::Result<()>;
    fn ParkDirect(&mut self, pparkaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ParkIndirect(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Unpark(&mut self) -> ::windows::core::Result<()>;
    fn SetQOS(&mut self, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::Result<()>;
    fn Pickup(&mut self, pgroupid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Dial(&mut self, pdestaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Finish(&mut self, finishmode: FINISH_MODE) -> ::windows::core::Result<()>;
    fn RemoveFromConference(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicCallControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITBasicCallControl_Vtbl {
        unsafe extern "system" fn Connect<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Answer<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Answer().into()
        }
        unsafe extern "system" fn Disconnect<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: DISCONNECT_CODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(::core::mem::transmute_copy(&code)).into()
        }
        unsafe extern "system" fn Hold<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fhold: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hold(::core::mem::transmute_copy(&fhold)).into()
        }
        unsafe extern "system" fn HandoffDirect<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandoffDirect(::core::mem::transmute_copy(&papplicationname)).into()
        }
        unsafe extern "system" fn HandoffIndirect<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandoffIndirect(::core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn Conference<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Conference(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Transfer<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Transfer(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn BlindTransfer<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BlindTransfer(::core::mem::transmute_copy(&pdestaddress)).into()
        }
        unsafe extern "system" fn SwapHold<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SwapHold(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn ParkDirect<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparkaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParkDirect(::core::mem::transmute_copy(&pparkaddress)).into()
        }
        unsafe extern "system" fn ParkIndirect<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnondiraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParkIndirect() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnondiraddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unpark<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unpark().into()
        }
        unsafe extern "system" fn SetQOS<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQOS(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&servicelevel)).into()
        }
        unsafe extern "system" fn Pickup<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroupid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pickup(::core::mem::transmute_copy(&pgroupid)).into()
        }
        unsafe extern "system" fn Dial<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Dial(::core::mem::transmute_copy(&pdestaddress)).into()
        }
        unsafe extern "system" fn Finish<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finishmode: FINISH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&finishmode)).into()
        }
        unsafe extern "system" fn RemoveFromConference<Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromConference().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITBasicCallControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicCallControl2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITBasicCallControl_Impl {
    fn RequestTerminal(&mut self, bstrterminalclassguid: &super::super::Foundation::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
    fn SelectTerminalOnCall(&mut self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn UnselectTerminalOnCall(&mut self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicCallControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITBasicCallControl2_Vtbl {
        unsafe extern "system" fn RequestTerminal<Impl: ITBasicCallControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrterminalclassguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestTerminal(::core::mem::transmute_copy(&bstrterminalclassguid), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectTerminalOnCall<Impl: ITBasicCallControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectTerminalOnCall(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminalOnCall<Impl: ITBasicCallControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnselectTerminalOnCall(::core::mem::transmute(&pterminal)).into()
        }
        Self {
            base: ITBasicCallControl_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RequestTerminal: RequestTerminal::<Impl, IMPL_OFFSET>,
            SelectTerminalOnCall: SelectTerminalOnCall::<Impl, IMPL_OFFSET>,
            UnselectTerminalOnCall: UnselectTerminalOnCall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicCallControl2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITBasicCallControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallHub_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn EnumerateCalls(&mut self) -> ::windows::core::Result<IEnumCall>;
    fn Calls(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NumCalls(&mut self) -> ::windows::core::Result<i32>;
    fn State(&mut self) -> ::windows::core::Result<CALLHUB_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallHub_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHub_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallHub_Vtbl {
        unsafe extern "system" fn Clear<Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn EnumerateCalls<Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcalls: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Calls() {
                ::core::result::Result::Ok(ok__) => {
                    *pcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumCalls<Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut CALLHUB_STATE) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Clear: Clear::<Impl, IMPL_OFFSET>,
            EnumerateCalls: EnumerateCalls::<Impl, IMPL_OFFSET>,
            Calls: Calls::<Impl, IMPL_OFFSET>,
            NumCalls: NumCalls::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallHub as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallHubEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Event(&mut self) -> ::windows::core::Result<CALLHUB_EVENT>;
    fn CallHub(&mut self) -> ::windows::core::Result<ITCallHub>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallHubEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHubEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallHubEvent_Vtbl {
        unsafe extern "system" fn Event<Impl: ITCallHubEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut CALLHUB_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Impl: ITCallHubEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHub() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITCallHubEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Event: Event::<Impl, IMPL_OFFSET>,
            CallHub: CallHub::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallHubEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn CallState(&mut self) -> ::windows::core::Result<CALL_STATE>;
    fn Privilege(&mut self) -> ::windows::core::Result<CALL_PRIVILEGE>;
    fn CallHub(&mut self) -> ::windows::core::Result<ITCallHub>;
    fn CallInfoLong(&mut self, callinfolong: CALLINFO_LONG) -> ::windows::core::Result<i32>;
    fn SetCallInfoLong(&mut self, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::Result<()>;
    fn CallInfoString(&mut self, callinfostring: CALLINFO_STRING) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCallInfoString(&mut self, callinfostring: CALLINFO_STRING, pcallinfostring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CallInfoBuffer(&mut self, callinfobuffer: CALLINFO_BUFFER) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetCallInfoBuffer(&mut self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetCallInfoBuffer(&mut self, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetCallInfoBuffer2(&mut self, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows::core::Result<()>;
    fn ReleaseUserUserInfo(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallInfo_Vtbl {
        unsafe extern "system" fn Address<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallState<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallState() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivilege: *mut CALL_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Privilege() {
                ::core::result::Result::Ok(ok__) => {
                    *pprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHub() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallInfoLong<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallInfoLong(::core::mem::transmute_copy(&callinfolong)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcallinfolongval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoLong<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallInfoLong(::core::mem::transmute_copy(&callinfolong), ::core::mem::transmute_copy(&lcallinfolongval)).into()
        }
        unsafe extern "system" fn CallInfoString<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallInfoString(::core::mem::transmute_copy(&callinfostring)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfostring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoString<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, pcallinfostring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallInfoString(::core::mem::transmute_copy(&callinfostring), ::core::mem::transmute_copy(&pcallinfostring)).into()
        }
        unsafe extern "system" fn CallInfoBuffer<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfobuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoBuffer<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn GetCallInfoBuffer<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppcallinfobuffer)).into()
        }
        unsafe extern "system" fn SetCallInfoBuffer2<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallInfoBuffer2(::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn ReleaseUserUserInfo<Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseUserUserInfo().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
            SetCallInfoBuffer2: SetCallInfoBuffer2::<Impl, IMPL_OFFSET>,
            ReleaseUserUserInfo: ReleaseUserUserInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfo2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITCallInfo_Impl {
    fn EventFilter(&mut self, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows::core::Result<i16>;
    fn SetEventFilter(&mut self, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallInfo2_Vtbl {
        unsafe extern "system" fn EventFilter<Impl: ITCallInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *penable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITCallInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent), ::core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base: ITCallInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventFilter: EventFilter::<Impl, IMPL_OFFSET>,
            SetEventFilter: SetEventFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfo2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITCallInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfoChangeEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Cause(&mut self) -> ::windows::core::Result<CALLINFOCHANGE_CAUSE>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfoChangeEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfoChangeEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallInfoChangeEvent_Vtbl {
        unsafe extern "system" fn Call<Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcic: *mut CALLINFOCHANGE_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Cause: Cause::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfoChangeEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallMediaEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Event(&mut self) -> ::windows::core::Result<CALL_MEDIA_EVENT>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Stream(&mut self) -> ::windows::core::Result<ITStream>;
    fn Cause(&mut self) -> ::windows::core::Result<CALL_MEDIA_EVENT_CAUSE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallMediaEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallMediaEvent_Vtbl {
        unsafe extern "system" fn Call<Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallmediaevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Stream: Stream::<Impl, IMPL_OFFSET>,
            Cause: Cause::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallMediaEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallNotificationEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Event(&mut self) -> ::windows::core::Result<CALL_NOTIFICATION_EVENT>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallNotificationEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallNotificationEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallNotificationEvent_Vtbl {
        unsafe extern "system" fn Call<Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallnotificationevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallNotificationEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallStateEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn State(&mut self) -> ::windows::core::Result<CALL_STATE>;
    fn Cause(&mut self) -> ::windows::core::Result<CALL_STATE_EVENT_CAUSE>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallStateEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallStateEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallStateEvent_Vtbl {
        unsafe extern "system" fn Call<Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcec: *mut CALL_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Cause: Cause::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallStateEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallingCard_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PermanentCardID(&mut self) -> ::windows::core::Result<i32>;
    fn NumberOfDigits(&mut self) -> ::windows::core::Result<i32>;
    fn Options(&mut self) -> ::windows::core::Result<i32>;
    fn CardName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SameAreaDialingRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LongDistanceDialingRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InternationalDialingRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallingCard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCard_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCallingCard_Vtbl {
        unsafe extern "system" fn PermanentCardID<Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermanentCardID() {
                ::core::result::Result::Ok(ok__) => {
                    *plcardid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfDigits<Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldigits: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *pldigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *ploptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardName<Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcardname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcardname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SameAreaDialingRule<Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SameAreaDialingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pprule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceDialingRule<Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongDistanceDialingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pprule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternationalDialingRule<Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITCallingCard as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCollection_Vtbl {
        unsafe extern "system" fn Count<Impl: ITCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *lcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ITCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ITCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCollection2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITCollection_Impl {
    fn Add(&mut self, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&mut self, index: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCollection2_Vtbl {
        unsafe extern "system" fn Add<Impl: ITCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pvariant)).into()
        }
        unsafe extern "system" fn Remove<Impl: ITCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        Self { base: ITCollection_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Add: Add::<Impl, IMPL_OFFSET>, Remove: Remove::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCollection2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCustomTone_Impl: Sized + super::super::System::Com::IDispatch_Impl {
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
impl ITCustomTone_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITCustomTone_Vtbl {
        unsafe extern "system" fn Frequency<Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frequency() {
                ::core::result::Result::Ok(ok__) => {
                    *plfrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfrequency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrequency(::core::mem::transmute_copy(&lfrequency)).into()
        }
        unsafe extern "system" fn CadenceOn<Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcadenceon: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CadenceOn() {
                ::core::result::Result::Ok(ok__) => {
                    *plcadenceon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOn<Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cadenceon: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCadenceOn(::core::mem::transmute_copy(&cadenceon)).into()
        }
        unsafe extern "system" fn CadenceOff<Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcadenceoff: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CadenceOff() {
                ::core::result::Result::Ok(ok__) => {
                    *plcadenceoff = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOff<Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcadenceoff: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCadenceOff(::core::mem::transmute_copy(&lcadenceoff)).into()
        }
        unsafe extern "system" fn Volume<Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITCustomTone as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDetectTone_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&mut self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn Duration(&mut self) -> ::windows::core::Result<i32>;
    fn SetDuration(&mut self, lduration: i32) -> ::windows::core::Result<()>;
    fn Frequency(&mut self, index: i32) -> ::windows::core::Result<i32>;
    fn SetFrequency(&mut self, index: i32, lfrequency: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDetectTone_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectTone_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDetectTone_Vtbl {
        unsafe extern "system" fn AppSpecific<Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn Duration<Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *plduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn Frequency<Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frequency(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *plfrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, lfrequency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrequency(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&lfrequency)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AppSpecific: AppSpecific::<Impl, IMPL_OFFSET>,
            SetAppSpecific: SetAppSpecific::<Impl, IMPL_OFFSET>,
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Frequency: Frequency::<Impl, IMPL_OFFSET>,
            SetFrequency: SetFrequency::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDetectTone as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitDetectionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Digit(&mut self) -> ::windows::core::Result<u8>;
    fn DigitMode(&mut self) -> ::windows::core::Result<i32>;
    fn TickCount(&mut self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitDetectionEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitDetectionEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDigitDetectionEvent_Vtbl {
        unsafe extern "system" fn Call<Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digit<Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucdigit: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Digit() {
                ::core::result::Result::Ok(ok__) => {
                    *pucdigit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitMode<Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigitmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DigitMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pdigitmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Digit: Digit::<Impl, IMPL_OFFSET>,
            DigitMode: DigitMode::<Impl, IMPL_OFFSET>,
            TickCount: TickCount::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitDetectionEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitGenerationEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn GenerationTermination(&mut self) -> ::windows::core::Result<i32>;
    fn TickCount(&mut self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitGenerationEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitGenerationEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDigitGenerationEvent_Vtbl {
        unsafe extern "system" fn Call<Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerationTermination<Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plgenerationtermination: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerationTermination() {
                ::core::result::Result::Ok(ok__) => {
                    *plgenerationtermination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            GenerationTermination: GenerationTermination::<Impl, IMPL_OFFSET>,
            TickCount: TickCount::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitGenerationEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitsGatheredEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Digits(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GatherTermination(&mut self) -> ::windows::core::Result<TAPI_GATHERTERM>;
    fn TickCount(&mut self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitsGatheredEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitsGatheredEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDigitsGatheredEvent_Vtbl {
        unsafe extern "system" fn Call<Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digits<Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdigits: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Digits() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GatherTermination<Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgathertermination: *mut TAPI_GATHERTERM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GatherTermination() {
                ::core::result::Result::Ok(ok__) => {
                    *pgathertermination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Digits: Digits::<Impl, IMPL_OFFSET>,
            GatherTermination: GatherTermination::<Impl, IMPL_OFFSET>,
            TickCount: TickCount::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitsGatheredEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DirectoryType(&mut self) -> ::windows::core::Result<DIRECTORY_TYPE>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsDynamic(&mut self) -> ::windows::core::Result<i16>;
    fn DefaultObjectTTL(&mut self) -> ::windows::core::Result<i32>;
    fn SetDefaultObjectTTL(&mut self, ttl: i32) -> ::windows::core::Result<()>;
    fn EnableAutoRefresh(&mut self, fenable: i16) -> ::windows::core::Result<()>;
    fn Connect(&mut self, fsecure: i16) -> ::windows::core::Result<()>;
    fn Bind(&mut self, pdomainname: &super::super::Foundation::BSTR, pusername: &super::super::Foundation::BSTR, ppassword: &super::super::Foundation::BSTR, lflags: i32) -> ::windows::core::Result<()>;
    fn AddDirectoryObject(&mut self, pdirectoryobject: &::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn ModifyDirectoryObject(&mut self, pdirectoryobject: &::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn RefreshDirectoryObject(&mut self, pdirectoryobject: &::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn DeleteDirectoryObject(&mut self, pdirectoryobject: &::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn DirectoryObjects(&mut self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDirectoryObjects(&mut self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IEnumDirectoryObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectory_Vtbl {
        unsafe extern "system" fn DirectoryType<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectorytype: *mut DIRECTORY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirectorytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDynamic<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdynamic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDynamic() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdynamic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultObjectTTL<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultObjectTTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultObjectTTL<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ttl: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultObjectTTL(::core::mem::transmute_copy(&ttl)).into()
        }
        unsafe extern "system" fn EnableAutoRefresh<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableAutoRefresh(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn Connect<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsecure: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&fsecure)).into()
        }
        unsafe extern "system" fn Bind<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Bind(::core::mem::transmute_copy(&pdomainname), ::core::mem::transmute_copy(&pusername), ::core::mem::transmute_copy(&ppassword), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddDirectoryObject<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn ModifyDirectoryObject<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifyDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn RefreshDirectoryObject<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn DeleteDirectoryObject<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn DirectoryObjects<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryObjects(::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDirectoryObjects<Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenumobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITDirectory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ObjectType(&mut self) -> ::windows::core::Result<DIRECTORY_OBJECT_TYPE>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DialableAddrs(&mut self, dwaddresstype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDialableAddrs(&mut self, dwaddresstype: u32) -> ::windows::core::Result<IEnumDialableAddrs>;
    fn SecurityDescriptor(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(&mut self, psecdes: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryObject_Vtbl {
        unsafe extern "system" fn ObjectType<Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    *pobjecttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&pname)).into()
        }
        unsafe extern "system" fn DialableAddrs<Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddresstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DialableAddrs(::core::mem::transmute_copy(&dwaddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDialableAddrs<Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddresstype: u32, ppenumdialableaddrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateDialableAddrs(::core::mem::transmute_copy(&dwaddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdialableaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecdes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecdes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecdes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute(&psecdes)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITDirectoryObject as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectConference_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Protocol(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Originator(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOriginator(&mut self, poriginator: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AdvertisingScope(&mut self) -> ::windows::core::Result<RND_ADVERTISING_SCOPE>;
    fn SetAdvertisingScope(&mut self, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::Result<()>;
    fn Url(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetUrl(&mut self, purl: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, pdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsEncrypted(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsEncrypted(&mut self, fencrypted: i16) -> ::windows::core::Result<()>;
    fn StartTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStartTime(&mut self, date: f64) -> ::windows::core::Result<()>;
    fn StopTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetStopTime(&mut self, date: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectConference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryObjectConference_Vtbl {
        unsafe extern "system" fn Protocol<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Originator<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginator: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Originator() {
                ::core::result::Result::Ok(ok__) => {
                    *pporiginator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginator<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poriginator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOriginator(::core::mem::transmute_copy(&poriginator)).into()
        }
        unsafe extern "system" fn AdvertisingScope<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisingScope() {
                ::core::result::Result::Ok(ok__) => {
                    *padvertisingscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisingScope<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvertisingScope(::core::mem::transmute_copy(&advertisingscope)).into()
        }
        unsafe extern "system" fn Url<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *ppurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUrl(::core::mem::transmute_copy(&purl)).into()
        }
        unsafe extern "system" fn Description<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn IsEncrypted<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfencrypted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEncrypted() {
                ::core::result::Result::Ok(ok__) => {
                    *pfencrypted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEncrypted<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fencrypted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEncrypted(::core::mem::transmute_copy(&fencrypted)).into()
        }
        unsafe extern "system" fn StartTime<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn StopTime<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopTime<Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStopTime(::core::mem::transmute_copy(&date)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITDirectoryObjectConference as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectUser_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IPPhonePrimary(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetIPPhonePrimary(&mut self, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectUser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDirectoryObjectUser_Vtbl {
        unsafe extern "system" fn IPPhonePrimary<Impl: ITDirectoryObjectUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IPPhonePrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPPhonePrimary<Impl: ITDirectoryObjectUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIPPhonePrimary(::core::mem::transmute_copy(&pname)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IPPhonePrimary: IPPhonePrimary::<Impl, IMPL_OFFSET>,
            SetIPPhonePrimary: SetIPPhonePrimary::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObjectUser as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDispatchMapper_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn QueryDispatchInterface(&mut self, piid: &super::super::Foundation::BSTR, pinterfacetomap: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDispatchMapper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDispatchMapper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITDispatchMapper_Vtbl {
        unsafe extern "system" fn QueryDispatchInterface<Impl: ITDispatchMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pinterfacetomap: ::windows::core::RawPtr, ppreturnedinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryDispatchInterface(::core::mem::transmute_copy(&piid), ::core::mem::transmute(&pinterfacetomap)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreturnedinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            QueryDispatchInterface: QueryDispatchInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDispatchMapper as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITFileTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Track(&mut self) -> ::windows::core::Result<ITFileTrack>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn State(&mut self) -> ::windows::core::Result<TERMINAL_MEDIA_STATE>;
    fn Cause(&mut self) -> ::windows::core::Result<FT_STATE_EVENT_CAUSE>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITFileTerminalEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITFileTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track<Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrackterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Track() {
                ::core::result::Result::Ok(ok__) => {
                    *pptrackterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcause: *mut FT_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcause = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Track: Track::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Cause: Cause::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITFileTerminalEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITFileTrack_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Format(&mut self) -> ::windows::core::Result<*mut super::super::Media::DirectShow::AM_MEDIA_TYPE>;
    fn SetFormat(&mut self, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
    fn ControllingTerminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn AudioFormatForScripting(&mut self) -> ::windows::core::Result<ITScriptableAudioFormat>;
    fn SetAudioFormatForScripting(&mut self, paudioformat: &::core::option::Option<ITScriptableAudioFormat>) -> ::windows::core::Result<()>;
    fn EmptyAudioFormatForScripting(&mut self) -> ::windows::core::Result<ITScriptableAudioFormat>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITFileTrack_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrack_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITFileTrack_Vtbl {
        unsafe extern "system" fn Format<Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&pmt)).into()
        }
        unsafe extern "system" fn ControllingTerminal<Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontrollingterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControllingTerminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontrollingterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatForScripting<Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioFormatForScripting() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaudioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioFormatForScripting<Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudioformat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioFormatForScripting(::core::mem::transmute(&paudioformat)).into()
        }
        unsafe extern "system" fn EmptyAudioFormatForScripting<Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Format: Format::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            ControllingTerminal: ControllingTerminal::<Impl, IMPL_OFFSET>,
            AudioFormatForScripting: AudioFormatForScripting::<Impl, IMPL_OFFSET>,
            SetAudioFormatForScripting: SetAudioFormatForScripting::<Impl, IMPL_OFFSET>,
            EmptyAudioFormatForScripting: EmptyAudioFormatForScripting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITFileTrack as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITForwardInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetNumRingsNoAnswer(&mut self, lnumrings: i32) -> ::windows::core::Result<()>;
    fn NumRingsNoAnswer(&mut self) -> ::windows::core::Result<i32>;
    fn SetForwardType(&mut self, forwardtype: i32, pdestaddress: &super::super::Foundation::BSTR, pcalleraddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ForwardTypeDestination(&mut self, forwardtype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ForwardTypeCaller(&mut self, forwardtype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetForwardType(&mut self, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITForwardInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITForwardInformation_Vtbl {
        unsafe extern "system" fn SetNumRingsNoAnswer<Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumrings: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumRingsNoAnswer(::core::mem::transmute_copy(&lnumrings)).into()
        }
        unsafe extern "system" fn NumRingsNoAnswer<Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnumrings: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumRingsNoAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    *plnumrings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForwardType<Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForwardType(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&pdestaddress), ::core::mem::transmute_copy(&pcalleraddress)).into()
        }
        unsafe extern "system" fn ForwardTypeDestination<Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardTypeDestination(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeCaller<Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardTypeCaller(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcalleraddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForwardType<Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForwardType(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&ppcalleraddress)).into()
        }
        unsafe extern "system" fn Clear<Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITForwardInformation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITForwardInformation2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITForwardInformation_Impl {
    fn SetForwardType2(&mut self, forwardtype: i32, pdestaddress: &super::super::Foundation::BSTR, destaddresstype: i32, pcalleraddress: &super::super::Foundation::BSTR, calleraddresstype: i32) -> ::windows::core::Result<()>;
    fn GetForwardType2(&mut self, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut super::super::Foundation::BSTR, pcalleraddresstype: *mut i32) -> ::windows::core::Result<()>;
    fn ForwardTypeDestinationAddressType(&mut self, forwardtype: i32) -> ::windows::core::Result<i32>;
    fn ForwardTypeCallerAddressType(&mut self, forwardtype: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITForwardInformation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITForwardInformation2_Vtbl {
        unsafe extern "system" fn SetForwardType2<Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destaddresstype: i32, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, calleraddresstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForwardType2(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&pdestaddress), ::core::mem::transmute_copy(&destaddresstype), ::core::mem::transmute_copy(&pcalleraddress), ::core::mem::transmute_copy(&calleraddresstype)).into()
        }
        unsafe extern "system" fn GetForwardType2<Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut super::super::Foundation::BSTR, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForwardType2(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&pdestaddresstype), ::core::mem::transmute_copy(&ppcalleraddress), ::core::mem::transmute_copy(&pcalleraddresstype)).into()
        }
        unsafe extern "system" fn ForwardTypeDestinationAddressType<Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardTypeDestinationAddressType(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdestaddresstype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeCallerAddressType<Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
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
            base: ITForwardInformation_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetForwardType2: SetForwardType2::<Impl, IMPL_OFFSET>,
            GetForwardType2: GetForwardType2::<Impl, IMPL_OFFSET>,
            ForwardTypeDestinationAddressType: ForwardTypeDestinationAddressType::<Impl, IMPL_OFFSET>,
            ForwardTypeCallerAddressType: ForwardTypeCallerAddressType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITForwardInformation2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITForwardInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITILSConfig_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Port(&mut self) -> ::windows::core::Result<i32>;
    fn SetPort(&mut self, port: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITILSConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITILSConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITILSConfig_Vtbl {
        unsafe extern "system" fn Port<Impl: ITILSConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Port() {
                ::core::result::Result::Ok(ok__) => {
                    *pport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Impl: ITILSConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, port: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPort(::core::mem::transmute_copy(&port)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Port: Port::<Impl, IMPL_OFFSET>,
            SetPort: SetPort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITILSConfig as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITLegacyAddressMediaControl_Impl: Sized {
    fn GetID(&mut self, pdeviceclass: &super::super::Foundation::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetDevConfig(&mut self, pdeviceclass: &super::super::Foundation::BSTR, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetDevConfig(&mut self, pdeviceclass: &super::super::Foundation::BSTR, dwsize: u32, pdeviceconfig: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITLegacyAddressMediaControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyAddressMediaControl_Vtbl {
        unsafe extern "system" fn GetID<Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetID(::core::mem::transmute_copy(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn GetDevConfig<Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevConfig(::core::mem::transmute_copy(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceconfig)).into()
        }
        unsafe extern "system" fn SetDevConfig<Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsize: u32, pdeviceconfig: *const u8) -> ::windows::core::HRESULT {
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
pub trait ITLegacyAddressMediaControl2_Impl: Sized + ITLegacyAddressMediaControl_Impl {
    fn ConfigDialog(&mut self, hwndowner: super::super::Foundation::HWND, pdeviceclass: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ConfigDialogEdit(&mut self, hwndowner: super::super::Foundation::HWND, pdeviceclass: &super::super::Foundation::BSTR, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITLegacyAddressMediaControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyAddressMediaControl2_Vtbl {
        unsafe extern "system" fn ConfigDialog<Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigDialog(::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&pdeviceclass)).into()
        }
        unsafe extern "system" fn ConfigDialogEdit<Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigDialogEdit(::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&pdeviceclass), ::core::mem::transmute_copy(&dwsizein), ::core::mem::transmute_copy(&pdeviceconfigin), ::core::mem::transmute_copy(&pdwsizeout), ::core::mem::transmute_copy(&ppdeviceconfigout)).into()
        }
        Self {
            base: ITLegacyAddressMediaControl_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ConfigDialog: ConfigDialog::<Impl, IMPL_OFFSET>,
            ConfigDialogEdit: ConfigDialogEdit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyAddressMediaControl2 as ::windows::core::Interface>::IID || iid == &<ITLegacyAddressMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyCallMediaControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DetectDigits(&mut self, digitmode: i32) -> ::windows::core::Result<()>;
    fn GenerateDigits(&mut self, pdigits: &super::super::Foundation::BSTR, digitmode: i32) -> ::windows::core::Result<()>;
    fn GetID(&mut self, pdeviceclass: &super::super::Foundation::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetMediaType(&mut self, lmediatype: i32) -> ::windows::core::Result<()>;
    fn MonitorMedia(&mut self, lmediatype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyCallMediaControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyCallMediaControl_Vtbl {
        unsafe extern "system" fn DetectDigits<Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectDigits(::core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GenerateDigits<Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateDigits(::core::mem::transmute_copy(&pdigits), ::core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GetID<Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetID(::core::mem::transmute_copy(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn SetMediaType<Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaType(::core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn MonitorMedia<Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MonitorMedia(::core::mem::transmute_copy(&lmediatype)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DetectDigits: DetectDigits::<Impl, IMPL_OFFSET>,
            GenerateDigits: GenerateDigits::<Impl, IMPL_OFFSET>,
            GetID: GetID::<Impl, IMPL_OFFSET>,
            SetMediaType: SetMediaType::<Impl, IMPL_OFFSET>,
            MonitorMedia: MonitorMedia::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyCallMediaControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyCallMediaControl2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITLegacyCallMediaControl_Impl {
    fn GenerateDigits2(&mut self, pdigits: &super::super::Foundation::BSTR, digitmode: i32, lduration: i32) -> ::windows::core::Result<()>;
    fn GatherDigits(&mut self, digitmode: i32, lnumdigits: i32, pterminationdigits: &super::super::Foundation::BSTR, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::Result<()>;
    fn DetectTones(&mut self, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::Result<()>;
    fn DetectTonesByCollection(&mut self, pdetecttonecollection: &::core::option::Option<ITCollection2>) -> ::windows::core::Result<()>;
    fn GenerateTone(&mut self, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::Result<()>;
    fn GenerateCustomTones(&mut self, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::Result<()>;
    fn GenerateCustomTonesByCollection(&mut self, pcustomtonecollection: &::core::option::Option<ITCollection2>, lduration: i32) -> ::windows::core::Result<()>;
    fn CreateDetectToneObject(&mut self) -> ::windows::core::Result<ITDetectTone>;
    fn CreateCustomToneObject(&mut self) -> ::windows::core::Result<ITCustomTone>;
    fn GetIDAsVariant(&mut self, bstrdeviceclass: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyCallMediaControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyCallMediaControl2_Vtbl {
        unsafe extern "system" fn GenerateDigits2<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateDigits2(::core::mem::transmute_copy(&pdigits), ::core::mem::transmute_copy(&digitmode), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GatherDigits<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitmode: i32, lnumdigits: i32, pterminationdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GatherDigits(::core::mem::transmute_copy(&digitmode), ::core::mem::transmute_copy(&lnumdigits), ::core::mem::transmute_copy(&pterminationdigits), ::core::mem::transmute_copy(&lfirstdigittimeout), ::core::mem::transmute_copy(&linterdigittimeout)).into()
        }
        unsafe extern "system" fn DetectTones<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectTones(::core::mem::transmute_copy(&ptonelist), ::core::mem::transmute_copy(&lnumtones)).into()
        }
        unsafe extern "system" fn DetectTonesByCollection<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetecttonecollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetectTonesByCollection(::core::mem::transmute(&pdetecttonecollection)).into()
        }
        unsafe extern "system" fn GenerateTone<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateTone(::core::mem::transmute_copy(&tonemode), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTones<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateCustomTones(::core::mem::transmute_copy(&ptonelist), ::core::mem::transmute_copy(&lnumtones), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTonesByCollection<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustomtonecollection: ::windows::core::RawPtr, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateCustomTonesByCollection(::core::mem::transmute(&pcustomtonecollection), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn CreateDetectToneObject<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdetecttone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDetectToneObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdetecttone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomToneObject<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcustomtone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCustomToneObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcustomtone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDAsVariant<Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardeviceid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: ITLegacyCallMediaControl_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITLegacyCallMediaControl2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITLegacyCallMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyWaveSupport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsFullDuplex(&mut self) -> ::windows::core::Result<FULLDUPLEX_SUPPORT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyWaveSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyWaveSupport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLegacyWaveSupport_Vtbl {
        unsafe extern "system" fn IsFullDuplex<Impl: ITLegacyWaveSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupport: *mut FULLDUPLEX_SUPPORT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFullDuplex() {
                ::core::result::Result::Ok(ok__) => {
                    *psupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsFullDuplex: IsFullDuplex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyWaveSupport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLocationInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
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
impl ITLocationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITLocationInfo_Vtbl {
        unsafe extern "system" fn PermanentLocationID<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllocationid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PermanentLocationID() {
                ::core::result::Result::Ok(ok__) => {
                    *pllocationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryCode<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plcountrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryID<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountryid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryID() {
                ::core::result::Result::Ok(ok__) => {
                    *plcountryid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *ploptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredCardID<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredCardID() {
                ::core::result::Result::Ok(ok__) => {
                    *plcardid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationName<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplocationname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationName() {
                ::core::result::Result::Ok(ok__) => {
                    *pplocationname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CityCode<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CityCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAccessCode<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAccessCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceAccessCode<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongDistanceAccessCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TollPrefixList<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptolllist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TollPrefixList() {
                ::core::result::Result::Ok(ok__) => {
                    *pptolllist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelCallWaitingCode<Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITLocationInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait ITMSPAddress_Impl: Sized {
    fn Initialize(&mut self, hevent: *const i32) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
    fn CreateMSPCall(&mut self, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ShutdownMSPCall(&mut self, pstreamcontrol: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ReceiveTSPData(&mut self, pmspcall: &::core::option::Option<::windows::core::IUnknown>, pbuffer: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn GetEvent(&mut self, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::Result<()>;
}
impl ITMSPAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMSPAddress_Vtbl {
        unsafe extern "system" fn Initialize<Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Shutdown<Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn CreateMSPCall<Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut ::core::ffi::c_void, ppstreamcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMSPCall(::core::mem::transmute_copy(&hcall), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&dwmediatype), ::core::mem::transmute(&pouterunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstreamcontrol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownMSPCall<Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstreamcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutdownMSPCall(::core::mem::transmute(&pstreamcontrol)).into()
        }
        unsafe extern "system" fn ReceiveTSPData<Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmspcall: *mut ::core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReceiveTSPData(::core::mem::transmute(&pmspcall), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetEvent<Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::HRESULT {
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
pub trait ITMediaControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn MediaState(&mut self) -> ::windows::core::Result<TERMINAL_MEDIA_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaControl_Vtbl {
        unsafe extern "system" fn Start<Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Pause<Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn MediaState<Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            MediaState: MediaState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaPlayback_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetPlayList(&mut self, playlistvariant: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlayList(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaPlayback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaPlayback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaPlayback_Vtbl {
        unsafe extern "system" fn SetPlayList<Impl: ITMediaPlayback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, playlistvariant: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayList(::core::mem::transmute_copy(&playlistvariant)).into()
        }
        unsafe extern "system" fn PlayList<Impl: ITMediaPlayback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylistvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPlayList: SetPlayList::<Impl, IMPL_OFFSET>,
            PlayList: PlayList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaPlayback as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaRecord_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetFileName(&mut self, bstrfilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FileName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaRecord_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaRecord_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaRecord_Vtbl {
        unsafe extern "system" fn SetFileName<Impl: ITMediaRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileName(::core::mem::transmute_copy(&bstrfilename)).into()
        }
        unsafe extern "system" fn FileName<Impl: ITMediaRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetFileName: SetFileName::<Impl, IMPL_OFFSET>,
            FileName: FileName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaRecord as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaSupport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaTypes(&mut self) -> ::windows::core::Result<i32>;
    fn QueryMediaType(&mut self, lmediatype: i32) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaSupport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMediaSupport_Vtbl {
        unsafe extern "system" fn MediaTypes<Impl: ITMediaSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMediaType<Impl: ITMediaSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, pfsupport: *mut i16) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MediaTypes: MediaTypes::<Impl, IMPL_OFFSET>,
            QueryMediaType: QueryMediaType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaSupport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMultiTrackTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TrackTerminals(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateTrackTerminals(&mut self) -> ::windows::core::Result<IEnumTerminal>;
    fn CreateTrackTerminal(&mut self, mediatype: i32, terminaldirection: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
    fn MediaTypesInUse(&mut self) -> ::windows::core::Result<i32>;
    fn DirectionsInUse(&mut self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
    fn RemoveTrackTerminal(&mut self, ptrackterminaltoremove: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMultiTrackTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITMultiTrackTerminal_Vtbl {
        unsafe extern "system" fn TrackTerminals<Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTrackTerminals<Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTrackTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrackTerminal<Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTrackTerminal(::core::mem::transmute_copy(&mediatype), ::core::mem::transmute_copy(&terminaldirection)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypesInUse<Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypesinuse: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaTypesInUse() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypesinuse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectionsInUse<Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldirectionsinused: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectionsInUse() {
                ::core::result::Result::Ok(ok__) => {
                    *pldirectionsinused = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrackTerminal<Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrackterminaltoremove: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTrackTerminal(::core::mem::transmute(&ptrackterminaltoremove)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TrackTerminals: TrackTerminals::<Impl, IMPL_OFFSET>,
            EnumerateTrackTerminals: EnumerateTrackTerminals::<Impl, IMPL_OFFSET>,
            CreateTrackTerminal: CreateTrackTerminal::<Impl, IMPL_OFFSET>,
            MediaTypesInUse: MediaTypesInUse::<Impl, IMPL_OFFSET>,
            DirectionsInUse: DirectionsInUse::<Impl, IMPL_OFFSET>,
            RemoveTrackTerminal: RemoveTrackTerminal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMultiTrackTerminal as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhone_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Open(&mut self, privilege: PHONE_PRIVILEGE) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Addresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateAddresses(&mut self) -> ::windows::core::Result<IEnumAddress>;
    fn PhoneCapsLong(&mut self, pclcap: PHONECAPS_LONG) -> ::windows::core::Result<i32>;
    fn PhoneCapsString(&mut self, pcscap: PHONECAPS_STRING) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Terminals(&mut self, paddress: &::core::option::Option<ITAddress>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateTerminals(&mut self, paddress: &::core::option::Option<ITAddress>) -> ::windows::core::Result<IEnumTerminal>;
    fn ButtonMode(&mut self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_MODE>;
    fn SetButtonMode(&mut self, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::Result<()>;
    fn ButtonFunction(&mut self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_FUNCTION>;
    fn SetButtonFunction(&mut self, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::Result<()>;
    fn ButtonText(&mut self, lbuttonid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetButtonText(&mut self, lbuttonid: i32, bstrbuttontext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
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
    fn SetDisplay(&mut self, lrow: i32, lcolumn: i32, bstrdisplay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PreferredAddresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePreferredAddresses(&mut self) -> ::windows::core::Result<IEnumAddress>;
    fn DeviceSpecific(&mut self, pparams: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn DeviceSpecificVariant(&mut self, vardevspecificbytearray: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NegotiateExtVersion(&mut self, llowversion: i32, lhighversion: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhone_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPhone_Vtbl {
        unsafe extern "system" fn Open<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privilege: PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&privilege)).into()
        }
        unsafe extern "system" fn Close<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Addresses<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *paddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneCapsLong<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneCapsLong(::core::mem::transmute_copy(&pclcap)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneCapsString<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcscap: PHONECAPS_STRING, ppcapability: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneCapsString(::core::mem::transmute_copy(&pcscap)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminals(::core::mem::transmute(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *pterminals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTerminals(::core::mem::transmute(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonMode<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonMode(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbuttonmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonMode<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonMode(::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&buttonmode)).into()
        }
        unsafe extern "system" fn ButtonFunction<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonFunction(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbuttonfunction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonFunction<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonFunction(::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&buttonfunction)).into()
        }
        unsafe extern "system" fn ButtonText<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, ppbuttontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonText(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuttontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonText<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, bstrbuttontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonText(::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&bstrbuttontext)).into()
        }
        unsafe extern "system" fn ButtonState<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonState(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbuttonstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HookSwitchState(::core::mem::transmute_copy(&hookswitchdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *phookswitchstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHookSwitchState<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHookSwitchState(::core::mem::transmute_copy(&hookswitchdevice), ::core::mem::transmute_copy(&hookswitchstate)).into()
        }
        unsafe extern "system" fn SetRingMode<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRingMode(::core::mem::transmute_copy(&lringmode)).into()
        }
        unsafe extern "system" fn RingMode<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plringmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingVolume<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRingVolume(::core::mem::transmute_copy(&lringvolume)).into()
        }
        unsafe extern "system" fn RingVolume<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *plringvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivilege: *mut PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Privilege() {
                ::core::result::Result::Ok(ok__) => {
                    *pprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneCapsBuffer<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPhoneCapsBuffer(::core::mem::transmute_copy(&pcbcaps), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppphonecapsbuffer)).into()
        }
        unsafe extern "system" fn PhoneCapsBuffer<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneCapsBuffer(::core::mem::transmute_copy(&pcbcaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LampMode<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LampMode(::core::mem::transmute_copy(&llampid)) {
                ::core::result::Result::Ok(ok__) => {
                    *plampmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLampMode<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLampMode(::core::mem::transmute_copy(&llampid), ::core::mem::transmute_copy(&lampmode)).into()
        }
        unsafe extern "system" fn Display<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisplay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplay<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrow: i32, lcolumn: i32, bstrdisplay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplay(::core::mem::transmute_copy(&lrow), ::core::mem::transmute_copy(&lcolumn), ::core::mem::transmute_copy(&bstrdisplay)).into()
        }
        unsafe extern "system" fn PreferredAddresses<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *paddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredAddresses<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePreferredAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSpecific<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSpecific(::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceSpecificVariant(::core::mem::transmute_copy(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITPhone as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneDeviceSpecificEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Phone(&mut self) -> ::windows::core::Result<ITPhone>;
    fn lParam1(&mut self) -> ::windows::core::Result<i32>;
    fn lParam2(&mut self) -> ::windows::core::Result<i32>;
    fn lParam3(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhoneDeviceSpecificEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneDeviceSpecificEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPhoneDeviceSpecificEvent_Vtbl {
        unsafe extern "system" fn Phone<Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam1() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).lParam2() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Phone: Phone::<Impl, IMPL_OFFSET>,
            lParam1: lParam1::<Impl, IMPL_OFFSET>,
            lParam2: lParam2::<Impl, IMPL_OFFSET>,
            lParam3: lParam3::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhoneDeviceSpecificEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
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
impl ITPhoneEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPhoneEvent_Vtbl {
        unsafe extern "system" fn Phone<Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut PHONE_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonState<Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HookSwitchState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchDevice<Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HookSwitchDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *pdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingMode<Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plringmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonLampId<Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbuttonlampid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonLampId() {
                ::core::result::Result::Ok(ok__) => {
                    *plbuttonlampid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberGathered<Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberGathered() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITPhoneEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPluggableTerminalClassInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Company(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Version(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TerminalClass(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CLSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Direction(&mut self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
    fn MediaTypes(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPluggableTerminalClassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalClassInfo_Vtbl {
        unsafe extern "system" fn Name<Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Company<Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompany: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Company() {
                ::core::result::Result::Ok(ok__) => {
                    *pcompany = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *pversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalClass() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminalclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypes<Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITPluggableTerminalClassInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalEventSink_Impl: Sized {
    fn FireEvent(&mut self, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalEventSink_Vtbl {
        unsafe extern "system" fn FireEvent<Impl: ITPluggableTerminalEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FireEvent(::core::mem::transmute_copy(&pmspeventinfo)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FireEvent: FireEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalEventSink as ::windows::core::Interface>::IID
    }
}
pub trait ITPluggableTerminalEventSinkRegistration_Impl: Sized {
    fn RegisterSink(&mut self, peventsink: &::core::option::Option<ITPluggableTerminalEventSink>) -> ::windows::core::Result<()>;
    fn UnregisterSink(&mut self) -> ::windows::core::Result<()>;
}
impl ITPluggableTerminalEventSinkRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalEventSinkRegistration_Vtbl {
        unsafe extern "system" fn RegisterSink<Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterSink(::core::mem::transmute(&peventsink)).into()
        }
        unsafe extern "system" fn UnregisterSink<Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait ITPluggableTerminalSuperclassInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CLSID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPluggableTerminalSuperclassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalSuperclassInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPluggableTerminalSuperclassInfo_Vtbl {
        unsafe extern "system" fn Name<Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            CLSID: CLSID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalSuperclassInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPrivateEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn CallHub(&mut self) -> ::windows::core::Result<ITCallHub>;
    fn EventCode(&mut self) -> ::windows::core::Result<i32>;
    fn EventInterface(&mut self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPrivateEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPrivateEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITPrivateEvent_Vtbl {
        unsafe extern "system" fn Address<Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHub() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventCode<Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pleventcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pleventcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterface<Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Address: Address::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            CallHub: CallHub::<Impl, IMPL_OFFSET>,
            EventCode: EventCode::<Impl, IMPL_OFFSET>,
            EventInterface: EventInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPrivateEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQOSEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Event(&mut self) -> ::windows::core::Result<QOS_EVENT>;
    fn MediaType(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQOSEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQOSEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITQOSEvent_Vtbl {
        unsafe extern "system" fn Call<Impl: ITQOSEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITQOSEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqosevent: *mut QOS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pqosevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: ITQOSEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQOSEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
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
impl ITQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITQueue_Vtbl {
        unsafe extern "system" fn SetMeasurementPeriod<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMeasurementPeriod(::core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasurementPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *plperiod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsQueued<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallsQueued() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCallsQueued<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentCallsQueued() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsAbandoned<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallsAbandoned() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedIn<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallsFlowedIn() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedOut<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCallsFlowedOut() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestEverWaitTime<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LongestEverWaitTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLongestWaitTime<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentLongestWaitTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWaitTime<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AverageWaitTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalDisposition<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalDisposition() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITQueue as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQueueEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Queue(&mut self) -> ::windows::core::Result<ITQueue>;
    fn Event(&mut self) -> ::windows::core::Result<ACDQUEUE_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQueueEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQueueEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITQueueEvent_Vtbl {
        unsafe extern "system" fn Queue<Impl: ITQueueEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Queue() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITQueueEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDQUEUE_EVENT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Queue: Queue::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQueueEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRendezvous_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DefaultDirectories(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDefaultDirectories(&mut self) -> ::windows::core::Result<IEnumDirectory>;
    fn CreateDirectory(&mut self, directorytype: DIRECTORY_TYPE, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITDirectory>;
    fn CreateDirectoryObject(&mut self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITDirectoryObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRendezvous_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRendezvous_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITRendezvous_Vtbl {
        unsafe extern "system" fn DefaultDirectories<Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultDirectories() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDefaultDirectories<Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdirectory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateDefaultDirectories() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdirectory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectory<Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directorytype: DIRECTORY_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirectory(::core::mem::transmute_copy(&directorytype), ::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectoryObject<Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdirectoryobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DefaultDirectories: DefaultDirectories::<Impl, IMPL_OFFSET>,
            EnumerateDefaultDirectories: EnumerateDefaultDirectories::<Impl, IMPL_OFFSET>,
            CreateDirectory: CreateDirectory::<Impl, IMPL_OFFSET>,
            CreateDirectoryObject: CreateDirectoryObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRendezvous as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRequest_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MakeCall(&mut self, pdestaddress: &super::super::Foundation::BSTR, pappname: &super::super::Foundation::BSTR, pcalledparty: &super::super::Foundation::BSTR, pcomment: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITRequest_Vtbl {
        unsafe extern "system" fn MakeCall<Impl: ITRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalledparty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeCall(::core::mem::transmute_copy(&pdestaddress), ::core::mem::transmute_copy(&pappname), ::core::mem::transmute_copy(&pcalledparty), ::core::mem::transmute_copy(&pcomment)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), MakeCall: MakeCall::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRequest as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRequestEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RegistrationInstance(&mut self) -> ::windows::core::Result<i32>;
    fn RequestMode(&mut self) -> ::windows::core::Result<i32>;
    fn DestAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AppName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CalledParty(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Comment(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRequestEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITRequestEvent_Vtbl {
        unsafe extern "system" fn RegistrationInstance<Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plregistrationinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistrationInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plregistrationinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMode<Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrequestmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plrequestmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestAddress<Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppName<Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppappname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalledParty<Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcalledparty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalledParty() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcalledparty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegistrationInstance: RegistrationInstance::<Impl, IMPL_OFFSET>,
            RequestMode: RequestMode::<Impl, IMPL_OFFSET>,
            DestAddress: DestAddress::<Impl, IMPL_OFFSET>,
            AppName: AppName::<Impl, IMPL_OFFSET>,
            CalledParty: CalledParty::<Impl, IMPL_OFFSET>,
            Comment: Comment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRequestEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITScriptableAudioFormat_Impl: Sized + super::super::System::Com::IDispatch_Impl {
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
impl ITScriptableAudioFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITScriptableAudioFormat_Vtbl {
        unsafe extern "system" fn Channels<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Channels() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannels(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn SamplesPerSec<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SamplesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSamplesPerSec(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn AvgBytesPerSec<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvgBytesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAvgBytesPerSec(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BlockAlign<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockAlign() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBlockAlign(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BitsPerSample<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitsPerSample() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitsPerSample(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn FormatTag<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatTag() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormatTag(::core::mem::transmute_copy(&nnewval)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITScriptableAudioFormat as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStaticAudioTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WaveId(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStaticAudioTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStaticAudioTerminal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITStaticAudioTerminal_Vtbl {
        unsafe extern "system" fn WaveId<Impl: ITStaticAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaveId() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaveid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), WaveId: WaveId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStaticAudioTerminal as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaType(&mut self) -> ::windows::core::Result<i32>;
    fn Direction(&mut self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StartStream(&mut self) -> ::windows::core::Result<()>;
    fn PauseStream(&mut self) -> ::windows::core::Result<()>;
    fn StopStream(&mut self) -> ::windows::core::Result<()>;
    fn SelectTerminal(&mut self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn UnselectTerminal(&mut self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn EnumerateTerminals(&mut self) -> ::windows::core::Result<IEnumTerminal>;
    fn Terminals(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITStream_Vtbl {
        unsafe extern "system" fn MediaType<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptd: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartStream<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartStream().into()
        }
        unsafe extern "system" fn PauseStream<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseStream().into()
        }
        unsafe extern "system" fn StopStream<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopStream().into()
        }
        unsafe extern "system" fn SelectTerminal<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnselectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStreamControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateStream(&mut self, lmediatype: i32, td: TERMINAL_DIRECTION) -> ::windows::core::Result<ITStream>;
    fn RemoveStream(&mut self, pstream: &::core::option::Option<ITStream>) -> ::windows::core::Result<()>;
    fn EnumerateStreams(&mut self) -> ::windows::core::Result<IEnumStream>;
    fn Streams(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStreamControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITStreamControl_Vtbl {
        unsafe extern "system" fn CreateStream<Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStream(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&td)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn EnumerateStreams<Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Streams<Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateStream: CreateStream::<Impl, IMPL_OFFSET>,
            RemoveStream: RemoveStream::<Impl, IMPL_OFFSET>,
            EnumerateStreams: EnumerateStreams::<Impl, IMPL_OFFSET>,
            Streams: Streams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStreamControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITSubStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartSubStream(&mut self) -> ::windows::core::Result<()>;
    fn PauseSubStream(&mut self) -> ::windows::core::Result<()>;
    fn StopSubStream(&mut self) -> ::windows::core::Result<()>;
    fn SelectTerminal(&mut self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn UnselectTerminal(&mut self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn EnumerateTerminals(&mut self) -> ::windows::core::Result<IEnumTerminal>;
    fn Terminals(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Stream(&mut self) -> ::windows::core::Result<ITStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITSubStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSubStream_Vtbl {
        unsafe extern "system" fn StartSubStream<Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartSubStream().into()
        }
        unsafe extern "system" fn PauseSubStream<Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseSubStream().into()
        }
        unsafe extern "system" fn StopSubStream<Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopSubStream().into()
        }
        unsafe extern "system" fn SelectTerminal<Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnselectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITSubStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITSubStreamControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateSubStream(&mut self) -> ::windows::core::Result<ITSubStream>;
    fn RemoveSubStream(&mut self, psubstream: &::core::option::Option<ITSubStream>) -> ::windows::core::Result<()>;
    fn EnumerateSubStreams(&mut self) -> ::windows::core::Result<IEnumSubStream>;
    fn SubStreams(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITSubStreamControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITSubStreamControl_Vtbl {
        unsafe extern "system" fn CreateSubStream<Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSubStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubStream<Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubStream(::core::mem::transmute(&psubstream)).into()
        }
        unsafe extern "system" fn EnumerateSubStreams<Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateSubStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsubstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubStreams<Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateSubStream: CreateSubStream::<Impl, IMPL_OFFSET>,
            RemoveSubStream: RemoveSubStream::<Impl, IMPL_OFFSET>,
            EnumerateSubStreams: EnumerateSubStreams::<Impl, IMPL_OFFSET>,
            SubStreams: SubStreams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSubStreamControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPI_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
    fn Addresses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateAddresses(&mut self) -> ::windows::core::Result<IEnumAddress>;
    fn RegisterCallNotifications(&mut self, paddress: &::core::option::Option<ITAddress>, fmonitor: i16, fowner: i16, lmediatypes: i32, lcallbackinstance: i32) -> ::windows::core::Result<i32>;
    fn UnregisterNotifications(&mut self, lregister: i32) -> ::windows::core::Result<()>;
    fn CallHubs(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCallHubs(&mut self) -> ::windows::core::Result<IEnumCallHub>;
    fn SetCallHubTracking(&mut self, paddresses: &super::super::System::Com::VARIANT, btracking: i16) -> ::windows::core::Result<()>;
    fn EnumeratePrivateTAPIObjects(&mut self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
    fn PrivateTAPIObjects(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RegisterRequestRecipient(&mut self, lregistrationinstance: i32, lrequestmode: i32, fenable: i16) -> ::windows::core::Result<()>;
    fn SetAssistedTelephonyPriority(&mut self, pappfilename: &super::super::Foundation::BSTR, fpriority: i16) -> ::windows::core::Result<()>;
    fn SetApplicationPriority(&mut self, pappfilename: &super::super::Foundation::BSTR, lmediatype: i32, fpriority: i16) -> ::windows::core::Result<()>;
    fn SetEventFilter(&mut self, lfiltermask: i32) -> ::windows::core::Result<()>;
    fn EventFilter(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPI_Vtbl {
        unsafe extern "system" fn Initialize<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn Shutdown<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn Addresses<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallNotifications<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, fmonitor: i16, fowner: i16, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterCallNotifications(::core::mem::transmute(&paddress), ::core::mem::transmute_copy(&fmonitor), ::core::mem::transmute_copy(&fowner), ::core::mem::transmute_copy(&lmediatypes), ::core::mem::transmute_copy(&lcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *plregister = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNotifications<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lregister: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterNotifications(::core::mem::transmute_copy(&lregister)).into()
        }
        unsafe extern "system" fn CallHubs<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallHubs() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallHubs<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateCallHubs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHubTracking<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, btracking: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallHubTracking(::core::mem::transmute_copy(&paddresses), ::core::mem::transmute_copy(&btracking)).into()
        }
        unsafe extern "system" fn EnumeratePrivateTAPIObjects<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePrivateTAPIObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateTAPIObjects<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateTAPIObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRequestRecipient<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lregistrationinstance: i32, lrequestmode: i32, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterRequestRecipient(::core::mem::transmute_copy(&lregistrationinstance), ::core::mem::transmute_copy(&lrequestmode), ::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SetAssistedTelephonyPriority<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAssistedTelephonyPriority(::core::mem::transmute_copy(&pappfilename), ::core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetApplicationPriority<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationPriority(::core::mem::transmute_copy(&pappfilename), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfiltermask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&lfiltermask)).into()
        }
        unsafe extern "system" fn EventFilter<Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfiltermask: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<ITTAPI as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPI2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITTAPI_Impl {
    fn Phones(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePhones(&mut self) -> ::windows::core::Result<IEnumPhone>;
    fn CreateEmptyCollectionObject(&mut self) -> ::windows::core::Result<ITCollection2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPI2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPI2_Vtbl {
        unsafe extern "system" fn Phones<Impl: ITTAPI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phones() {
                ::core::result::Result::Ok(ok__) => {
                    *pphones = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Impl: ITTAPI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePhones() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEmptyCollectionObject<Impl: ITTAPI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ITTAPI_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Phones: Phones::<Impl, IMPL_OFFSET>,
            EnumeratePhones: EnumeratePhones::<Impl, IMPL_OFFSET>,
            CreateEmptyCollectionObject: CreateEmptyCollectionObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPI2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITTAPI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPICallCenter_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumerateAgentHandlers(&mut self) -> ::windows::core::Result<IEnumAgentHandler>;
    fn AgentHandlers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPICallCenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPICallCenter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPICallCenter_Vtbl {
        unsafe extern "system" fn EnumerateAgentHandlers<Impl: ITTAPICallCenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateAgentHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumhandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentHandlers<Impl: ITTAPICallCenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumerateAgentHandlers: EnumerateAgentHandlers::<Impl, IMPL_OFFSET>,
            AgentHandlers: AgentHandlers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPICallCenter as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIDispatchEventNotification_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIDispatchEventNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIDispatchEventNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIDispatchEventNotification_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIDispatchEventNotification as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIEventNotification_Impl: Sized {
    fn Event(&mut self, tapievent: TAPI_EVENT, pevent: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIEventNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIEventNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIEventNotification_Vtbl {
        unsafe extern "system" fn Event<Impl: ITTAPIEventNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITTAPIObjectEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TAPIObject(&mut self) -> ::windows::core::Result<ITTAPI>;
    fn Event(&mut self) -> ::windows::core::Result<TAPIOBJECT_EVENT>;
    fn Address(&mut self) -> ::windows::core::Result<ITAddress>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIObjectEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIObjectEvent_Vtbl {
        unsafe extern "system" fn TAPIObject<Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TAPIObject() {
                ::core::result::Result::Ok(ok__) => {
                    *pptapiobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut TAPIOBJECT_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            TAPIObject: TAPIObject::<Impl, IMPL_OFFSET>,
            Event: Event::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIObjectEvent2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITTAPIObjectEvent_Impl {
    fn Phone(&mut self) -> ::windows::core::Result<ITPhone>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIObjectEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTAPIObjectEvent2_Vtbl {
        unsafe extern "system" fn Phone<Impl: ITTAPIObjectEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITTAPIObjectEvent_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Phone: Phone::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITTAPIObjectEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTTSTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTTSTerminalEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTTSTerminalEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTTSTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTTSTerminalEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&mut self) -> ::windows::core::Result<TERMINAL_STATE>;
    fn TerminalType(&mut self) -> ::windows::core::Result<TERMINAL_TYPE>;
    fn TerminalClass(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MediaType(&mut self) -> ::windows::core::Result<i32>;
    fn Direction(&mut self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTerminal_Vtbl {
        unsafe extern "system" fn Name<Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalstate: *mut TERMINAL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminalstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalType<Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TERMINAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminalClass() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminalclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            TerminalType: TerminalType::<Impl, IMPL_OFFSET>,
            TerminalClass: TerminalClass::<Impl, IMPL_OFFSET>,
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminal as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalSupport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StaticTerminals(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateStaticTerminals(&mut self) -> ::windows::core::Result<IEnumTerminal>;
    fn DynamicTerminalClasses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDynamicTerminalClasses(&mut self) -> ::windows::core::Result<IEnumTerminalClass>;
    fn CreateTerminal(&mut self, pterminalclass: &super::super::Foundation::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
    fn GetDefaultStaticTerminal(&mut self, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTerminalSupport_Vtbl {
        unsafe extern "system" fn StaticTerminals<Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaticTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateStaticTerminals<Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateStaticTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminalenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicTerminalClasses<Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DynamicTerminalClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDynamicTerminalClasses<Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerateDynamicTerminalClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminalclassenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTerminal<Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTerminal(::core::mem::transmute_copy(&pterminalclass), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultStaticTerminal<Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StaticTerminals: StaticTerminals::<Impl, IMPL_OFFSET>,
            EnumerateStaticTerminals: EnumerateStaticTerminals::<Impl, IMPL_OFFSET>,
            DynamicTerminalClasses: DynamicTerminalClasses::<Impl, IMPL_OFFSET>,
            EnumerateDynamicTerminalClasses: EnumerateDynamicTerminalClasses::<Impl, IMPL_OFFSET>,
            CreateTerminal: CreateTerminal::<Impl, IMPL_OFFSET>,
            GetDefaultStaticTerminal: GetDefaultStaticTerminal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminalSupport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalSupport2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITTerminalSupport_Impl {
    fn PluggableSuperclasses(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePluggableSuperclasses(&mut self) -> ::windows::core::Result<IEnumPluggableSuperclassInfo>;
    fn PluggableTerminalClasses(&mut self, bstrterminalsuperclass: &super::super::Foundation::BSTR, lmediatype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePluggableTerminalClasses(&mut self, iidterminalsuperclass: &::windows::core::GUID, lmediatype: i32) -> ::windows::core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalSupport2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITTerminalSupport2_Vtbl {
        unsafe extern "system" fn PluggableSuperclasses<Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PluggableSuperclasses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableSuperclasses<Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuperclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumeratePluggableSuperclasses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsuperclassenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PluggableTerminalClasses<Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrterminalsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PluggableTerminalClasses(::core::mem::transmute_copy(&bstrterminalsuperclass), ::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableTerminalClasses<Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iidterminalsuperclass: ::windows::core::GUID, lmediatype: i32, ppclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ITTerminalSupport_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PluggableSuperclasses: PluggableSuperclasses::<Impl, IMPL_OFFSET>,
            EnumeratePluggableSuperclasses: EnumeratePluggableSuperclasses::<Impl, IMPL_OFFSET>,
            PluggableTerminalClasses: PluggableTerminalClasses::<Impl, IMPL_OFFSET>,
            EnumeratePluggableTerminalClasses: EnumeratePluggableTerminalClasses::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminalSupport2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITTerminalSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITToneDetectionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn AppSpecific(&mut self) -> ::windows::core::Result<i32>;
    fn TickCount(&mut self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITToneDetectionEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITToneDetectionEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITToneDetectionEvent_Vtbl {
        unsafe extern "system" fn Call<Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Call: Call::<Impl, IMPL_OFFSET>,
            AppSpecific: AppSpecific::<Impl, IMPL_OFFSET>,
            TickCount: TickCount::<Impl, IMPL_OFFSET>,
            CallbackInstance: CallbackInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITToneDetectionEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITToneTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&mut self) -> ::windows::core::Result<ITTerminal>;
    fn Call(&mut self) -> ::windows::core::Result<ITCallInfo>;
    fn Error(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITToneTerminalEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITToneTerminalEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITToneTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Terminal: Terminal::<Impl, IMPL_OFFSET>,
            Call: Call::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITToneTerminalEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub trait ITnef_Impl: Sized {
    fn AddProps(&mut self, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::Result<()>;
    fn ExtractProps(&mut self, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()>;
    fn Finish(&mut self, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()>;
    fn OpenTaggedBody(&mut self, lpmessage: &::core::option::Option<super::super::System::AddressBook::IMessage>, ulflags: u32) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetProps(&mut self, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::Result<()>;
    fn EncodeRecips(&mut self, ulflags: u32, lprecipienttable: &::core::option::Option<super::super::System::AddressBook::IMAPITable>) -> ::windows::core::Result<()>;
    fn FinishComponent(&mut self, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
impl ITnef_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITnef_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITnef_Vtbl {
        unsafe extern "system" fn AddProps<Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddProps(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulelemid), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&lpproplist)).into()
        }
        unsafe extern "system" fn ExtractProps<Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExtractProps(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpproplist), ::core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn Finish<Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpkey), ::core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn OpenTaggedBody<Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmessage: ::windows::core::RawPtr, ulflags: u32, lppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTaggedBody(::core::mem::transmute(&lpmessage), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProps<Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProps(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulelemid), ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpprops)).into()
        }
        unsafe extern "system" fn EncodeRecips<Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lprecipienttable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EncodeRecips(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute(&lprecipienttable)).into()
        }
        unsafe extern "system" fn FinishComponent<Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
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
