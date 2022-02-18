#[cfg(feature = "Win32_System_Com")]
pub trait IEnumACDGroup_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITACDGroup>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumACDGroup>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumACDGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumACDGroup_Impl, const OFFSET: isize>() -> IEnumACDGroup_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumACDGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAddress_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITAddress>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumAddress>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAddress_Impl, const OFFSET: isize>() -> IEnumAddress_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgent_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITAgent>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumAgent>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgent_Impl, const OFFSET: isize>() -> IEnumAgent_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgentHandler_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITAgentHandler>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumAgentHandler>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>() -> IEnumAgentHandler_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAgentHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgentSession_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITAgentSession>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumAgentSession>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumAgentSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentSession_Impl, const OFFSET: isize>() -> IEnumAgentSession_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumAgentSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumBstr_Impl: Sized {
    fn Next(&self, celt: u32, ppstrings: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumBstr>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumBstr_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBstr_Impl, const OFFSET: isize>() -> IEnumBstr_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppstrings: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstrings), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumBstr_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumBstr as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCall_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITCallInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumCall>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCall_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCall_Impl, const OFFSET: isize>() -> IEnumCall_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCall as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCallHub_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITCallHub>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumCallHub>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCallHub_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallHub_Impl, const OFFSET: isize>() -> IEnumCallHub_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCallHub as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCallingCard_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITCallingCard>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumCallingCard>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumCallingCard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallingCard_Impl, const OFFSET: isize>() -> IEnumCallingCard_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCallingCard as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumDialableAddrs_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDialableAddrs>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumDialableAddrs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>() -> IEnumDialableAddrs_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDialableAddrs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDialableAddrs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumDirectory_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITDirectory>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDirectory>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumDirectory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectory_Impl, const OFFSET: isize>() -> IEnumDirectory_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDirectory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumDirectoryObject_Impl: Sized {
    fn Next(&self, celt: u32, pval: *mut ::core::option::Option<ITDirectoryObject>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumDirectoryObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumDirectoryObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>() -> IEnumDirectoryObject_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pval: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDirectoryObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumLocation_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITLocationInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumLocation>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumLocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumLocation_Impl, const OFFSET: isize>() -> IEnumLocation_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumLocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumLocation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMcastScope_Impl: Sized {
    fn Next(&self, celt: u32, ppscopes: *mut ::core::option::Option<IMcastScope>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumMcastScope>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumMcastScope_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMcastScope_Impl, const OFFSET: isize>() -> IEnumMcastScope_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppscopes: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppscopes), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMcastScope as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPhone_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITPhone>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumPhone>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPhone_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPhone_Impl, const OFFSET: isize>() -> IEnumPhone_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPhone as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPluggableSuperclassInfo_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITPluggableTerminalSuperclassInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumPluggableSuperclassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPluggableSuperclassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>() -> IEnumPluggableSuperclassInfo_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPluggableSuperclassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPluggableTerminalClassInfo_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITPluggableTerminalClassInfo>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumPluggableTerminalClassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>() -> IEnumPluggableTerminalClassInfo_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumPluggableTerminalClassInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumQueue_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITQueue>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumQueue>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumQueue_Impl, const OFFSET: isize>() -> IEnumQueue_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumStream_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITStream>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStream_Impl, const OFFSET: isize>() -> IEnumStream_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumSubStream_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITSubStream>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSubStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumSubStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubStream_Impl, const OFFSET: isize>() -> IEnumSubStream_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSubStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumTerminal_Impl: Sized {
    fn Next(&self, celt: u32, ppelements: *mut ::core::option::Option<ITTerminal>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumTerminal>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminal_Impl, const OFFSET: isize>() -> IEnumTerminal_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTerminal as ::windows::core::Interface>::IID
    }
}
pub trait IEnumTerminalClass_Impl: Sized {
    fn Next(&self, celt: u32, pelements: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumTerminalClass>;
}
impl IEnumTerminalClass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>() -> IEnumTerminalClass_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pelements: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pelements), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumTerminalClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumTerminalClass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastAddressAllocation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Scopes(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateScopes(&self) -> ::windows::core::Result<IEnumMcastScope>;
    fn RequestAddress(&self, pscope: &::core::option::Option<IMcastScope>, leasestarttime: f64, leasestoptime: f64, numaddresses: i32) -> ::windows::core::Result<IMcastLeaseInfo>;
    fn RenewAddress(&self, lreserved: i32, prenewrequest: &::core::option::Option<IMcastLeaseInfo>) -> ::windows::core::Result<IMcastLeaseInfo>;
    fn ReleaseAddress(&self, preleaserequest: &::core::option::Option<IMcastLeaseInfo>) -> ::windows::core::Result<()>;
    fn CreateLeaseInfo(&self, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const ::windows::core::PWSTR, prequestid: &::windows::core::PCWSTR, pserveraddress: &::windows::core::PCWSTR) -> ::windows::core::Result<IMcastLeaseInfo>;
    fn CreateLeaseInfoFromVariant(&self, leasestarttime: f64, leasestoptime: f64, vaddresses: &super::super::System::Com::VARIANT, prequestid: &super::super::Foundation::BSTR, pserveraddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<IMcastLeaseInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastAddressAllocation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>() -> IMcastAddressAllocation_Vtbl {
        unsafe extern "system" fn Scopes<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Scopes() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateScopes<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenummcastscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateScopes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenummcastscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscope: ::windows::core::RawPtr, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestAddress(::core::mem::transmute(&pscope), ::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute_copy(&numaddresses)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppleaseresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lreserved: i32, prenewrequest: ::windows::core::RawPtr, pprenewresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RenewAddress(::core::mem::transmute_copy(&lreserved), ::core::mem::transmute(&prenewrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprenewresponse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preleaserequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseAddress(::core::mem::transmute(&preleaserequest)).into()
        }
        unsafe extern "system" fn CreateLeaseInfo<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const ::windows::core::PWSTR, prequestid: ::windows::core::PCWSTR, pserveraddress: ::windows::core::PCWSTR, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLeaseInfo(::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute_copy(&dwnumaddresses), ::core::mem::transmute_copy(&ppaddresses), ::core::mem::transmute(&prequestid), ::core::mem::transmute(&pserveraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreleaserequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLeaseInfoFromVariant<Identity: ::windows::core::IUnknownImpl, Impl: IMcastAddressAllocation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, vaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, prequestid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pserveraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateLeaseInfoFromVariant(::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute(&vaddresses), ::core::mem::transmute(&prequestid), ::core::mem::transmute(&pserveraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreleaserequest = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Scopes: Scopes::<Identity, Impl, OFFSET>,
            EnumerateScopes: EnumerateScopes::<Identity, Impl, OFFSET>,
            RequestAddress: RequestAddress::<Identity, Impl, OFFSET>,
            RenewAddress: RenewAddress::<Identity, Impl, OFFSET>,
            ReleaseAddress: ReleaseAddress::<Identity, Impl, OFFSET>,
            CreateLeaseInfo: CreateLeaseInfo::<Identity, Impl, OFFSET>,
            CreateLeaseInfoFromVariant: CreateLeaseInfoFromVariant::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastAddressAllocation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastLeaseInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RequestID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LeaseStartTime(&self) -> ::windows::core::Result<f64>;
    fn SetLeaseStartTime(&self, time: f64) -> ::windows::core::Result<()>;
    fn LeaseStopTime(&self) -> ::windows::core::Result<f64>;
    fn SetLeaseStopTime(&self, time: f64) -> ::windows::core::Result<()>;
    fn AddressCount(&self) -> ::windows::core::Result<i32>;
    fn ServerAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TTL(&self) -> ::windows::core::Result<i32>;
    fn Addresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateAddresses(&self) -> ::windows::core::Result<IEnumBstr>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastLeaseInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>() -> IMcastLeaseInfo_Vtbl {
        unsafe extern "system" fn RequestID<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprequestid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestID() {
                ::core::result::Result::Ok(ok__) => {
                    *pprequestid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseStartTime<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LeaseStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStartTime<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLeaseStartTime(::core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn LeaseStopTime<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LeaseStopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStopTime<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLeaseStopTime(::core::mem::transmute_copy(&time)).into()
        }
        unsafe extern "system" fn AddressCount<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddressCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerAddress<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Addresses<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: ::windows::core::IUnknownImpl, Impl: IMcastLeaseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddresses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RequestID: RequestID::<Identity, Impl, OFFSET>,
            LeaseStartTime: LeaseStartTime::<Identity, Impl, OFFSET>,
            SetLeaseStartTime: SetLeaseStartTime::<Identity, Impl, OFFSET>,
            LeaseStopTime: LeaseStopTime::<Identity, Impl, OFFSET>,
            SetLeaseStopTime: SetLeaseStopTime::<Identity, Impl, OFFSET>,
            AddressCount: AddressCount::<Identity, Impl, OFFSET>,
            ServerAddress: ServerAddress::<Identity, Impl, OFFSET>,
            TTL: TTL::<Identity, Impl, OFFSET>,
            Addresses: Addresses::<Identity, Impl, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastLeaseInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMcastScope_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ScopeID(&self) -> ::windows::core::Result<i32>;
    fn ServerID(&self) -> ::windows::core::Result<i32>;
    fn InterfaceID(&self) -> ::windows::core::Result<i32>;
    fn ScopeDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TTL(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMcastScope_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMcastScope_Impl, const OFFSET: isize>() -> IMcastScope_Vtbl {
        unsafe extern "system" fn ScopeID<Identity: ::windows::core::IUnknownImpl, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScopeID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerID<Identity: ::windows::core::IUnknownImpl, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows::core::IUnknownImpl, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeDescription<Identity: ::windows::core::IUnknownImpl, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScopeDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Identity: ::windows::core::IUnknownImpl, Impl: IMcastScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ScopeID: ScopeID::<Identity, Impl, OFFSET>,
            ServerID: ServerID::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            ScopeDescription: ScopeDescription::<Identity, Impl, OFFSET>,
            TTL: TTL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMcastScope as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITACDGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnumerateQueues(&self) -> ::windows::core::Result<IEnumQueue>;
    fn Queues(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITACDGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroup_Impl, const OFFSET: isize>() -> ITACDGroup_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateQueues<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateQueues() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Queues<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Queues() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            EnumerateQueues: EnumerateQueues::<Identity, Impl, OFFSET>,
            Queues: Queues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITACDGroup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITACDGroupEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Group(&self) -> ::windows::core::Result<ITACDGroup>;
    fn Event(&self) -> ::windows::core::Result<ACDGROUP_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITACDGroupEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroupEvent_Impl, const OFFSET: isize>() -> ITACDGroupEvent_Vtbl {
        unsafe extern "system" fn Group<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITACDGroupEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDGROUP_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Group: Group::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITACDGroupEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait ITAMMediaFormat_Impl: Sized {
    fn MediaFormat(&self) -> ::windows::core::Result<*mut super::super::Media::DirectShow::AM_MEDIA_TYPE>;
    fn SetMediaFormat(&self, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ITAMMediaFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAMMediaFormat_Impl, const OFFSET: isize>() -> ITAMMediaFormat_Vtbl {
        unsafe extern "system" fn MediaFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITAMMediaFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITAMMediaFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMediaFormat(::core::mem::transmute_copy(&pmt)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MediaFormat: MediaFormat::<Identity, Impl, OFFSET>,
            SetMediaFormat: SetMediaFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAMMediaFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITASRTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&self) -> ::windows::core::Result<ITTerminal>;
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITASRTerminalEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>() -> ITASRTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Identity: ::windows::core::IUnknownImpl, Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: ITASRTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITASRTerminalEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddress_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn State(&self) -> ::windows::core::Result<ADDRESS_STATE>;
    fn AddressName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ServiceProviderName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TAPIObject(&self) -> ::windows::core::Result<ITTAPI>;
    fn CreateCall(&self, pdestaddress: &super::super::Foundation::BSTR, laddresstype: i32, lmediatypes: i32) -> ::windows::core::Result<ITBasicCallControl>;
    fn Calls(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCalls(&self) -> ::windows::core::Result<IEnumCall>;
    fn DialableAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateForwardInfoObject(&self) -> ::windows::core::Result<ITForwardInformation>;
    fn Forward(&self, pforwardinfo: &::core::option::Option<ITForwardInformation>, pcall: &::core::option::Option<ITBasicCallControl>) -> ::windows::core::Result<()>;
    fn CurrentForwardInfo(&self) -> ::windows::core::Result<ITForwardInformation>;
    fn SetMessageWaiting(&self, fmessagewaiting: i16) -> ::windows::core::Result<()>;
    fn MessageWaiting(&self) -> ::windows::core::Result<i16>;
    fn SetDoNotDisturb(&self, fdonotdisturb: i16) -> ::windows::core::Result<()>;
    fn DoNotDisturb(&self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>() -> ITAddress_Vtbl {
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddressstate: *mut ADDRESS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *paddressstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressName<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddressName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProviderName<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServiceProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TAPIObject<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TAPIObject() {
                ::core::result::Result::Ok(ok__) => {
                    *pptapiobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCall<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, laddresstype: i32, lmediatypes: i32, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCall(::core::mem::transmute(&pdestaddress), ::core::mem::transmute_copy(&laddresstype), ::core::mem::transmute_copy(&lmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Calls() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCalls<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialableAddress<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdialableaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DialableAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *pdialableaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForwardInfoObject<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateForwardInfoObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppforwardinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forward<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pforwardinfo: ::windows::core::RawPtr, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Forward(::core::mem::transmute(&pforwardinfo), ::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn CurrentForwardInfo<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentForwardInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppforwardinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageWaiting<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmessagewaiting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMessageWaiting(::core::mem::transmute_copy(&fmessagewaiting)).into()
        }
        unsafe extern "system" fn MessageWaiting<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmessagewaiting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MessageWaiting() {
                ::core::result::Result::Ok(ok__) => {
                    *pfmessagewaiting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotDisturb<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdonotdisturb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDoNotDisturb(::core::mem::transmute_copy(&fdonotdisturb)).into()
        }
        unsafe extern "system" fn DoNotDisturb<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdonotdisturb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DoNotDisturb() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdonotdisturb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            State: State::<Identity, Impl, OFFSET>,
            AddressName: AddressName::<Identity, Impl, OFFSET>,
            ServiceProviderName: ServiceProviderName::<Identity, Impl, OFFSET>,
            TAPIObject: TAPIObject::<Identity, Impl, OFFSET>,
            CreateCall: CreateCall::<Identity, Impl, OFFSET>,
            Calls: Calls::<Identity, Impl, OFFSET>,
            EnumerateCalls: EnumerateCalls::<Identity, Impl, OFFSET>,
            DialableAddress: DialableAddress::<Identity, Impl, OFFSET>,
            CreateForwardInfoObject: CreateForwardInfoObject::<Identity, Impl, OFFSET>,
            Forward: Forward::<Identity, Impl, OFFSET>,
            CurrentForwardInfo: CurrentForwardInfo::<Identity, Impl, OFFSET>,
            SetMessageWaiting: SetMessageWaiting::<Identity, Impl, OFFSET>,
            MessageWaiting: MessageWaiting::<Identity, Impl, OFFSET>,
            SetDoNotDisturb: SetDoNotDisturb::<Identity, Impl, OFFSET>,
            DoNotDisturb: DoNotDisturb::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddress as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddress2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITAddress_Impl {
    fn Phones(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePhones(&self) -> ::windows::core::Result<IEnumPhone>;
    fn GetPhoneFromTerminal(&self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<ITPhone>;
    fn PreferredPhones(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePreferredPhones(&self) -> ::windows::core::Result<IEnumPhone>;
    fn EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows::core::Result<i16>;
    fn SetEventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::Result<()>;
    fn DeviceSpecific(&self, pcall: &::core::option::Option<ITCallInfo>, pparams: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn DeviceSpecificVariant(&self, pcall: &::core::option::Option<ITCallInfo>, vardevspecificbytearray: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NegotiateExtVersion(&self, llowversion: i32, lhighversion: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddress2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>() -> ITAddress2_Vtbl {
        unsafe extern "system" fn Phones<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Phones() {
                ::core::result::Result::Ok(ok__) => {
                    *pphones = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumeratePhones() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneFromTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPhoneFromTerminal(::core::mem::transmute(&pterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPhones<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PreferredPhones() {
                ::core::result::Result::Ok(ok__) => {
                    *pphones = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredPhones<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumeratePreferredPhones() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventFilter<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *penable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn DeviceSpecific<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceSpecific(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceSpecificVariant(::core::mem::transmute(&pcall), ::core::mem::transmute(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Identity: ::windows::core::IUnknownImpl, Impl: ITAddress2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NegotiateExtVersion(::core::mem::transmute_copy(&llowversion), ::core::mem::transmute_copy(&lhighversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *plextversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITAddress_Vtbl::new::<Identity, Impl, OFFSET>(),
            Phones: Phones::<Identity, Impl, OFFSET>,
            EnumeratePhones: EnumeratePhones::<Identity, Impl, OFFSET>,
            GetPhoneFromTerminal: GetPhoneFromTerminal::<Identity, Impl, OFFSET>,
            PreferredPhones: PreferredPhones::<Identity, Impl, OFFSET>,
            EnumeratePreferredPhones: EnumeratePreferredPhones::<Identity, Impl, OFFSET>,
            EventFilter: EventFilter::<Identity, Impl, OFFSET>,
            SetEventFilter: SetEventFilter::<Identity, Impl, OFFSET>,
            DeviceSpecific: DeviceSpecific::<Identity, Impl, OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Identity, Impl, OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddress2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressCapabilities_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddressCapability(&self, addresscap: ADDRESS_CAPABILITY) -> ::windows::core::Result<i32>;
    fn AddressCapabilityString(&self, addresscapstring: ADDRESS_CAPABILITY_STRING) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CallTreatments(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCallTreatments(&self) -> ::windows::core::Result<IEnumBstr>;
    fn CompletionMessages(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCompletionMessages(&self) -> ::windows::core::Result<IEnumBstr>;
    fn DeviceClasses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDeviceClasses(&self) -> ::windows::core::Result<IEnumBstr>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>() -> ITAddressCapabilities_Vtbl {
        unsafe extern "system" fn AddressCapability<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddressCapability(::core::mem::transmute_copy(&addresscap)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressCapabilityString<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddressCapabilityString(::core::mem::transmute_copy(&addresscapstring)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapabilitystring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallTreatments<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallTreatments() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallTreatments<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcalltreatment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateCallTreatments() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcalltreatment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompletionMessages<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CompletionMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCompletionMessages<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcompletionmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateCompletionMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcompletionmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceClasses<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DeviceClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDeviceClasses<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdeviceclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateDeviceClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdeviceclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddressCapability: AddressCapability::<Identity, Impl, OFFSET>,
            AddressCapabilityString: AddressCapabilityString::<Identity, Impl, OFFSET>,
            CallTreatments: CallTreatments::<Identity, Impl, OFFSET>,
            EnumerateCallTreatments: EnumerateCallTreatments::<Identity, Impl, OFFSET>,
            CompletionMessages: CompletionMessages::<Identity, Impl, OFFSET>,
            EnumerateCompletionMessages: EnumerateCompletionMessages::<Identity, Impl, OFFSET>,
            DeviceClasses: DeviceClasses::<Identity, Impl, OFFSET>,
            EnumerateDeviceClasses: EnumerateDeviceClasses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressCapabilities as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressDeviceSpecificEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&self) -> ::windows::core::Result<ITAddress>;
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn lParam1(&self) -> ::windows::core::Result<i32>;
    fn lParam2(&self) -> ::windows::core::Result<i32>;
    fn lParam3(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressDeviceSpecificEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>() -> ITAddressDeviceSpecificEvent_Vtbl {
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).lParam1() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).lParam2() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).lParam3() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam3 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            lParam1: lParam1::<Identity, Impl, OFFSET>,
            lParam2: lParam2::<Identity, Impl, OFFSET>,
            lParam3: lParam3::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressDeviceSpecificEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&self) -> ::windows::core::Result<ITAddress>;
    fn Event(&self) -> ::windows::core::Result<ADDRESS_EVENT>;
    fn Terminal(&self) -> ::windows::core::Result<ITTerminal>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressEvent_Impl, const OFFSET: isize>() -> ITAddressEvent_Vtbl {
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ADDRESS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressTranslation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TranslateAddress(&self, paddresstotranslate: &super::super::Foundation::BSTR, lcard: i32, ltranslateoptions: i32) -> ::windows::core::Result<ITAddressTranslationInfo>;
    fn TranslateDialog(&self, hwndowner: isize, paddressin: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EnumerateLocations(&self) -> ::windows::core::Result<IEnumLocation>;
    fn Locations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCallingCards(&self) -> ::windows::core::Result<IEnumCallingCard>;
    fn CallingCards(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressTranslation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslation_Impl, const OFFSET: isize>() -> ITAddressTranslation_Vtbl {
        unsafe extern "system" fn TranslateAddress<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresstotranslate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcard: i32, ltranslateoptions: i32, pptranslated: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TranslateAddress(::core::mem::transmute(&paddresstotranslate), ::core::mem::transmute_copy(&lcard), ::core::mem::transmute_copy(&ltranslateoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptranslated = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateDialog<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: isize, paddressin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TranslateDialog(::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute(&paddressin)).into()
        }
        unsafe extern "system" fn EnumerateLocations<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumlocation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumlocation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locations<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Locations() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallingCards<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcallingcard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateCallingCards() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcallingcard = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallingCards<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallingCards() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TranslateAddress: TranslateAddress::<Identity, Impl, OFFSET>,
            TranslateDialog: TranslateDialog::<Identity, Impl, OFFSET>,
            EnumerateLocations: EnumerateLocations::<Identity, Impl, OFFSET>,
            Locations: Locations::<Identity, Impl, OFFSET>,
            EnumerateCallingCards: EnumerateCallingCards::<Identity, Impl, OFFSET>,
            CallingCards: CallingCards::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressTranslation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAddressTranslationInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DialableString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn DisplayableString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentCountryCode(&self) -> ::windows::core::Result<i32>;
    fn DestinationCountryCode(&self) -> ::windows::core::Result<i32>;
    fn TranslationResults(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAddressTranslationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>() -> ITAddressTranslationInfo_Vtbl {
        unsafe extern "system" fn DialableString<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdialablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DialableString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdialablestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayableString<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdisplayablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayableString() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdisplayablestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCountryCode<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *countrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationCountryCode<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestinationCountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *countrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslationResults<Identity: ::windows::core::IUnknownImpl, Impl: ITAddressTranslationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plresults: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TranslationResults() {
                ::core::result::Result::Ok(ok__) => {
                    *plresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DialableString: DialableString::<Identity, Impl, OFFSET>,
            DisplayableString: DisplayableString::<Identity, Impl, OFFSET>,
            CurrentCountryCode: CurrentCountryCode::<Identity, Impl, OFFSET>,
            DestinationCountryCode: DestinationCountryCode::<Identity, Impl, OFFSET>,
            TranslationResults: TranslationResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAddressTranslationInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumerateAgentSessions(&self) -> ::windows::core::Result<IEnumAgentSession>;
    fn CreateSession(&self, pacdgroup: &::core::option::Option<ITACDGroup>, paddress: &::core::option::Option<ITAddress>) -> ::windows::core::Result<ITAgentSession>;
    fn CreateSessionWithPIN(&self, pacdgroup: &::core::option::Option<ITACDGroup>, paddress: &::core::option::Option<ITAddress>, ppin: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITAgentSession>;
    fn ID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn User(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetState(&self, agentstate: AGENT_STATE) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<AGENT_STATE>;
    fn SetMeasurementPeriod(&self, lperiod: i32) -> ::windows::core::Result<()>;
    fn MeasurementPeriod(&self) -> ::windows::core::Result<i32>;
    fn OverallCallRate(&self) -> ::windows::core::Result<super::super::System::Com::CY>;
    fn NumberOfACDCalls(&self) -> ::windows::core::Result<i32>;
    fn NumberOfIncomingCalls(&self) -> ::windows::core::Result<i32>;
    fn NumberOfOutgoingCalls(&self) -> ::windows::core::Result<i32>;
    fn TotalACDTalkTime(&self) -> ::windows::core::Result<i32>;
    fn TotalACDCallTime(&self) -> ::windows::core::Result<i32>;
    fn TotalWrapUpTime(&self) -> ::windows::core::Result<i32>;
    fn AgentSessions(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>() -> ITAgent_Vtbl {
        unsafe extern "system" fn EnumerateAgentSessions<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateAgentSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumagentsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSession(::core::mem::transmute(&pacdgroup), ::core::mem::transmute(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppagentsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionWithPIN<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSessionWithPIN(::core::mem::transmute(&pacdgroup), ::core::mem::transmute(&paddress), ::core::mem::transmute(&ppin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppagentsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppuser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *ppuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agentstate: AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&agentstate)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pagentstate: *mut AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pagentstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeasurementPeriod<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMeasurementPeriod(::core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MeasurementPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *plperiod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallCallRate<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OverallCallRate() {
                ::core::result::Result::Ok(ok__) => {
                    *pcycallrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfACDCalls<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfACDCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfIncomingCalls<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfIncomingCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfOutgoingCalls<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfOutgoingCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDTalkTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalACDTalkTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pltalktime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDCallTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalACDCallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalWrapUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwrapuptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentSessions<Identity: ::windows::core::IUnknownImpl, Impl: ITAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AgentSessions() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumerateAgentSessions: EnumerateAgentSessions::<Identity, Impl, OFFSET>,
            CreateSession: CreateSession::<Identity, Impl, OFFSET>,
            CreateSessionWithPIN: CreateSessionWithPIN::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetMeasurementPeriod: SetMeasurementPeriod::<Identity, Impl, OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Identity, Impl, OFFSET>,
            OverallCallRate: OverallCallRate::<Identity, Impl, OFFSET>,
            NumberOfACDCalls: NumberOfACDCalls::<Identity, Impl, OFFSET>,
            NumberOfIncomingCalls: NumberOfIncomingCalls::<Identity, Impl, OFFSET>,
            NumberOfOutgoingCalls: NumberOfOutgoingCalls::<Identity, Impl, OFFSET>,
            TotalACDTalkTime: TotalACDTalkTime::<Identity, Impl, OFFSET>,
            TotalACDCallTime: TotalACDCallTime::<Identity, Impl, OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Identity, Impl, OFFSET>,
            AgentSessions: AgentSessions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Agent(&self) -> ::windows::core::Result<ITAgent>;
    fn Event(&self) -> ::windows::core::Result<AGENT_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentEvent_Impl, const OFFSET: isize>() -> ITAgentEvent_Vtbl {
        unsafe extern "system" fn Agent<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Agent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Agent: Agent::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentHandler_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreateAgent(&self) -> ::windows::core::Result<ITAgent>;
    fn CreateAgentWithID(&self, pid: &super::super::Foundation::BSTR, ppin: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITAgent>;
    fn EnumerateACDGroups(&self) -> ::windows::core::Result<IEnumACDGroup>;
    fn EnumerateUsableAddresses(&self) -> ::windows::core::Result<IEnumAddress>;
    fn ACDGroups(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn UsableAddresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandler_Impl, const OFFSET: isize>() -> ITAgentHandler_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgent<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAgent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgentWithID<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAgentWithID(::core::mem::transmute(&pid), ::core::mem::transmute(&ppin)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateACDGroups<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateACDGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumacdgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateUsableAddresses<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateUsableAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroups<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ACDGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsableAddresses<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UsableAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            CreateAgent: CreateAgent::<Identity, Impl, OFFSET>,
            CreateAgentWithID: CreateAgentWithID::<Identity, Impl, OFFSET>,
            EnumerateACDGroups: EnumerateACDGroups::<Identity, Impl, OFFSET>,
            EnumerateUsableAddresses: EnumerateUsableAddresses::<Identity, Impl, OFFSET>,
            ACDGroups: ACDGroups::<Identity, Impl, OFFSET>,
            UsableAddresses: UsableAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentHandler as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentHandlerEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AgentHandler(&self) -> ::windows::core::Result<ITAgentHandler>;
    fn Event(&self) -> ::windows::core::Result<AGENTHANDLER_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentHandlerEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandlerEvent_Impl, const OFFSET: isize>() -> ITAgentHandlerEvent_Vtbl {
        unsafe extern "system" fn AgentHandler<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandlerEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagenthandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AgentHandler() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagenthandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentHandlerEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENTHANDLER_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AgentHandler: AgentHandler::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentHandlerEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentSession_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Agent(&self) -> ::windows::core::Result<ITAgent>;
    fn Address(&self) -> ::windows::core::Result<ITAddress>;
    fn ACDGroup(&self) -> ::windows::core::Result<ITACDGroup>;
    fn SetState(&self, sessionstate: AGENT_SESSION_STATE) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<AGENT_SESSION_STATE>;
    fn SessionStartTime(&self) -> ::windows::core::Result<f64>;
    fn SessionDuration(&self) -> ::windows::core::Result<i32>;
    fn NumberOfCalls(&self) -> ::windows::core::Result<i32>;
    fn TotalTalkTime(&self) -> ::windows::core::Result<i32>;
    fn AverageTalkTime(&self) -> ::windows::core::Result<i32>;
    fn TotalCallTime(&self) -> ::windows::core::Result<i32>;
    fn AverageCallTime(&self) -> ::windows::core::Result<i32>;
    fn TotalWrapUpTime(&self) -> ::windows::core::Result<i32>;
    fn AverageWrapUpTime(&self) -> ::windows::core::Result<i32>;
    fn ACDCallRate(&self) -> ::windows::core::Result<super::super::System::Com::CY>;
    fn LongestTimeToAnswer(&self) -> ::windows::core::Result<i32>;
    fn AverageTimeToAnswer(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>() -> ITAgentSession_Vtbl {
        unsafe extern "system" fn Agent<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Agent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppagent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroup<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ACDGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *ppacdgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionstate: AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&sessionstate)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psessionstate: *mut AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *psessionstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdatesessionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdatesessionstart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionDuration<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *plduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfCalls<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTalkTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalTalkTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pltalktime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTalkTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AverageTalkTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pltalktime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalCallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageCallTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AverageCallTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalWrapUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwrapuptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWrapUpTime<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AverageWrapUpTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwrapuptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDCallRate<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ACDCallRate() {
                ::core::result::Result::Ok(ok__) => {
                    *pcycallrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestTimeToAnswer<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LongestTimeToAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    *planswertime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTimeToAnswer<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AverageTimeToAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    *planswertime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Agent: Agent::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            ACDGroup: ACDGroup::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SessionStartTime: SessionStartTime::<Identity, Impl, OFFSET>,
            SessionDuration: SessionDuration::<Identity, Impl, OFFSET>,
            NumberOfCalls: NumberOfCalls::<Identity, Impl, OFFSET>,
            TotalTalkTime: TotalTalkTime::<Identity, Impl, OFFSET>,
            AverageTalkTime: AverageTalkTime::<Identity, Impl, OFFSET>,
            TotalCallTime: TotalCallTime::<Identity, Impl, OFFSET>,
            AverageCallTime: AverageCallTime::<Identity, Impl, OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Identity, Impl, OFFSET>,
            AverageWrapUpTime: AverageWrapUpTime::<Identity, Impl, OFFSET>,
            ACDCallRate: ACDCallRate::<Identity, Impl, OFFSET>,
            LongestTimeToAnswer: LongestTimeToAnswer::<Identity, Impl, OFFSET>,
            AverageTimeToAnswer: AverageTimeToAnswer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentSession as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAgentSessionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Session(&self) -> ::windows::core::Result<ITAgentSession>;
    fn Event(&self) -> ::windows::core::Result<AGENT_SESSION_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAgentSessionEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSessionEvent_Impl, const OFFSET: isize>() -> ITAgentSessionEvent_Vtbl {
        unsafe extern "system" fn Session<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSessionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsession = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITAgentSessionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_SESSION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Session: Session::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAgentSessionEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait ITAllocatorProperties_Impl: Sized {
    fn SetAllocatorProperties(&self, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::Result<()>;
    fn GetAllocatorProperties(&self) -> ::windows::core::Result<super::super::Media::DirectShow::ALLOCATOR_PROPERTIES>;
    fn SetAllocateBuffers(&self, ballocbuffers: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateBuffers(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetBufferSize(&self, buffersize: u32) -> ::windows::core::Result<()>;
    fn GetBufferSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ITAllocatorProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>() -> ITAllocatorProperties_Vtbl {
        unsafe extern "system" fn SetAllocatorProperties<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllocatorProperties(::core::mem::transmute_copy(&pallocproperties)).into()
        }
        unsafe extern "system" fn GetAllocatorProperties<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllocatorProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pallocproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ballocbuffers: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllocateBuffers(::core::mem::transmute_copy(&ballocbuffers)).into()
        }
        unsafe extern "system" fn GetAllocateBuffers<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pballocbuffers: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllocateBuffers() {
                ::core::result::Result::Ok(ok__) => {
                    *pballocbuffers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferSize(::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows::core::IUnknownImpl, Impl: ITAllocatorProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pbuffersize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAllocatorProperties: SetAllocatorProperties::<Identity, Impl, OFFSET>,
            GetAllocatorProperties: GetAllocatorProperties::<Identity, Impl, OFFSET>,
            SetAllocateBuffers: SetAllocateBuffers::<Identity, Impl, OFFSET>,
            GetAllocateBuffers: GetAllocateBuffers::<Identity, Impl, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAllocatorProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITAutomatedPhoneControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartTone(&self, tone: PHONE_TONE, lduration: i32) -> ::windows::core::Result<()>;
    fn StopTone(&self) -> ::windows::core::Result<()>;
    fn Tone(&self) -> ::windows::core::Result<PHONE_TONE>;
    fn StartRinger(&self, lringmode: i32, lduration: i32) -> ::windows::core::Result<()>;
    fn StopRinger(&self) -> ::windows::core::Result<()>;
    fn Ringer(&self) -> ::windows::core::Result<i16>;
    fn SetPhoneHandlingEnabled(&self, fenabled: i16) -> ::windows::core::Result<()>;
    fn PhoneHandlingEnabled(&self) -> ::windows::core::Result<i16>;
    fn SetAutoEndOfNumberTimeout(&self, ltimeout: i32) -> ::windows::core::Result<()>;
    fn AutoEndOfNumberTimeout(&self) -> ::windows::core::Result<i32>;
    fn SetAutoDialtone(&self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoDialtone(&self) -> ::windows::core::Result<i16>;
    fn SetAutoStopTonesOnOnHook(&self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoStopTonesOnOnHook(&self) -> ::windows::core::Result<i16>;
    fn SetAutoStopRingOnOffHook(&self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoStopRingOnOffHook(&self) -> ::windows::core::Result<i16>;
    fn SetAutoKeypadTones(&self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoKeypadTones(&self) -> ::windows::core::Result<i16>;
    fn SetAutoKeypadTonesMinimumDuration(&self, lduration: i32) -> ::windows::core::Result<()>;
    fn AutoKeypadTonesMinimumDuration(&self) -> ::windows::core::Result<i32>;
    fn SetAutoVolumeControl(&self, fenabled: i16) -> ::windows::core::Result<()>;
    fn AutoVolumeControl(&self) -> ::windows::core::Result<i16>;
    fn SetAutoVolumeControlStep(&self, lstepsize: i32) -> ::windows::core::Result<()>;
    fn AutoVolumeControlStep(&self) -> ::windows::core::Result<i32>;
    fn SetAutoVolumeControlRepeatDelay(&self, ldelay: i32) -> ::windows::core::Result<()>;
    fn AutoVolumeControlRepeatDelay(&self) -> ::windows::core::Result<i32>;
    fn SetAutoVolumeControlRepeatPeriod(&self, lperiod: i32) -> ::windows::core::Result<()>;
    fn AutoVolumeControlRepeatPeriod(&self) -> ::windows::core::Result<i32>;
    fn SelectCall(&self, pcall: &::core::option::Option<ITCallInfo>, fselectdefaultterminals: i16) -> ::windows::core::Result<()>;
    fn UnselectCall(&self, pcall: &::core::option::Option<ITCallInfo>) -> ::windows::core::Result<()>;
    fn EnumerateSelectedCalls(&self) -> ::windows::core::Result<IEnumCall>;
    fn SelectedCalls(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITAutomatedPhoneControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>() -> ITAutomatedPhoneControl_Vtbl {
        unsafe extern "system" fn StartTone<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tone: PHONE_TONE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartTone(::core::mem::transmute_copy(&tone), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopTone<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopTone().into()
        }
        unsafe extern "system" fn Tone<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptone: *mut PHONE_TONE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tone() {
                ::core::result::Result::Ok(ok__) => {
                    *ptone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRinger<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartRinger(::core::mem::transmute_copy(&lringmode), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn StopRinger<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopRinger().into()
        }
        unsafe extern "system" fn Ringer<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfringing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Ringer() {
                ::core::result::Result::Ok(ok__) => {
                    *pfringing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhoneHandlingEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPhoneHandlingEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn PhoneHandlingEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhoneHandlingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoEndOfNumberTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ltimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoEndOfNumberTimeout(::core::mem::transmute_copy(&ltimeout)).into()
        }
        unsafe extern "system" fn AutoEndOfNumberTimeout<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltimeout: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoEndOfNumberTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *pltimeout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDialtone<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoDialtone(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoDialtone<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoDialtone() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopTonesOnOnHook<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoStopTonesOnOnHook(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopTonesOnOnHook<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoStopTonesOnOnHook() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopRingOnOffHook<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoStopRingOnOffHook(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoStopRingOnOffHook<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoStopRingOnOffHook() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTones<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoKeypadTones(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoKeypadTones<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoKeypadTones() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTonesMinimumDuration<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoKeypadTonesMinimumDuration(::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn AutoKeypadTonesMinimumDuration<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoKeypadTonesMinimumDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *plduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControl<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoVolumeControl(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn AutoVolumeControl<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoVolumeControl() {
                ::core::result::Result::Ok(ok__) => {
                    *fenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlStep<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lstepsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoVolumeControlStep(::core::mem::transmute_copy(&lstepsize)).into()
        }
        unsafe extern "system" fn AutoVolumeControlStep<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plstepsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoVolumeControlStep() {
                ::core::result::Result::Ok(ok__) => {
                    *plstepsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatDelay<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ldelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoVolumeControlRepeatDelay(::core::mem::transmute_copy(&ldelay)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatDelay<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoVolumeControlRepeatDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *pldelay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatPeriod<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoVolumeControlRepeatPeriod(::core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn AutoVolumeControlRepeatPeriod<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AutoVolumeControlRepeatPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *plperiod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectCall<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fselectdefaultterminals: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SelectCall(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&fselectdefaultterminals)).into()
        }
        unsafe extern "system" fn UnselectCall<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnselectCall(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn EnumerateSelectedCalls<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateSelectedCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedCalls<Identity: ::windows::core::IUnknownImpl, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SelectedCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartTone: StartTone::<Identity, Impl, OFFSET>,
            StopTone: StopTone::<Identity, Impl, OFFSET>,
            Tone: Tone::<Identity, Impl, OFFSET>,
            StartRinger: StartRinger::<Identity, Impl, OFFSET>,
            StopRinger: StopRinger::<Identity, Impl, OFFSET>,
            Ringer: Ringer::<Identity, Impl, OFFSET>,
            SetPhoneHandlingEnabled: SetPhoneHandlingEnabled::<Identity, Impl, OFFSET>,
            PhoneHandlingEnabled: PhoneHandlingEnabled::<Identity, Impl, OFFSET>,
            SetAutoEndOfNumberTimeout: SetAutoEndOfNumberTimeout::<Identity, Impl, OFFSET>,
            AutoEndOfNumberTimeout: AutoEndOfNumberTimeout::<Identity, Impl, OFFSET>,
            SetAutoDialtone: SetAutoDialtone::<Identity, Impl, OFFSET>,
            AutoDialtone: AutoDialtone::<Identity, Impl, OFFSET>,
            SetAutoStopTonesOnOnHook: SetAutoStopTonesOnOnHook::<Identity, Impl, OFFSET>,
            AutoStopTonesOnOnHook: AutoStopTonesOnOnHook::<Identity, Impl, OFFSET>,
            SetAutoStopRingOnOffHook: SetAutoStopRingOnOffHook::<Identity, Impl, OFFSET>,
            AutoStopRingOnOffHook: AutoStopRingOnOffHook::<Identity, Impl, OFFSET>,
            SetAutoKeypadTones: SetAutoKeypadTones::<Identity, Impl, OFFSET>,
            AutoKeypadTones: AutoKeypadTones::<Identity, Impl, OFFSET>,
            SetAutoKeypadTonesMinimumDuration: SetAutoKeypadTonesMinimumDuration::<Identity, Impl, OFFSET>,
            AutoKeypadTonesMinimumDuration: AutoKeypadTonesMinimumDuration::<Identity, Impl, OFFSET>,
            SetAutoVolumeControl: SetAutoVolumeControl::<Identity, Impl, OFFSET>,
            AutoVolumeControl: AutoVolumeControl::<Identity, Impl, OFFSET>,
            SetAutoVolumeControlStep: SetAutoVolumeControlStep::<Identity, Impl, OFFSET>,
            AutoVolumeControlStep: AutoVolumeControlStep::<Identity, Impl, OFFSET>,
            SetAutoVolumeControlRepeatDelay: SetAutoVolumeControlRepeatDelay::<Identity, Impl, OFFSET>,
            AutoVolumeControlRepeatDelay: AutoVolumeControlRepeatDelay::<Identity, Impl, OFFSET>,
            SetAutoVolumeControlRepeatPeriod: SetAutoVolumeControlRepeatPeriod::<Identity, Impl, OFFSET>,
            AutoVolumeControlRepeatPeriod: AutoVolumeControlRepeatPeriod::<Identity, Impl, OFFSET>,
            SelectCall: SelectCall::<Identity, Impl, OFFSET>,
            UnselectCall: UnselectCall::<Identity, Impl, OFFSET>,
            EnumerateSelectedCalls: EnumerateSelectedCalls::<Identity, Impl, OFFSET>,
            SelectedCalls: SelectedCalls::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITAutomatedPhoneControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicAudioTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetVolume(&self, lvolume: i32) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<i32>;
    fn SetBalance(&self, lbalance: i32) -> ::windows::core::Result<()>;
    fn Balance(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicAudioTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>() -> ITBasicAudioTerminal_Vtbl {
        unsafe extern "system" fn SetVolume<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn Volume<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBalance<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBalance(::core::mem::transmute_copy(&lbalance)).into()
        }
        unsafe extern "system" fn Balance<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Balance() {
                ::core::result::Result::Ok(ok__) => {
                    *plbalance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetBalance: SetBalance::<Identity, Impl, OFFSET>,
            Balance: Balance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicAudioTerminal as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicCallControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Connect(&self, fsync: i16) -> ::windows::core::Result<()>;
    fn Answer(&self) -> ::windows::core::Result<()>;
    fn Disconnect(&self, code: DISCONNECT_CODE) -> ::windows::core::Result<()>;
    fn Hold(&self, fhold: i16) -> ::windows::core::Result<()>;
    fn HandoffDirect(&self, papplicationname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn HandoffIndirect(&self, lmediatype: i32) -> ::windows::core::Result<()>;
    fn Conference(&self, pcall: &::core::option::Option<ITBasicCallControl>, fsync: i16) -> ::windows::core::Result<()>;
    fn Transfer(&self, pcall: &::core::option::Option<ITBasicCallControl>, fsync: i16) -> ::windows::core::Result<()>;
    fn BlindTransfer(&self, pdestaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SwapHold(&self, pcall: &::core::option::Option<ITBasicCallControl>) -> ::windows::core::Result<()>;
    fn ParkDirect(&self, pparkaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ParkIndirect(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Unpark(&self) -> ::windows::core::Result<()>;
    fn SetQOS(&self, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::Result<()>;
    fn Pickup(&self, pgroupid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Dial(&self, pdestaddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Finish(&self, finishmode: FINISH_MODE) -> ::windows::core::Result<()>;
    fn RemoveFromConference(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicCallControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>() -> ITBasicCallControl_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Answer<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Answer().into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: DISCONNECT_CODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect(::core::mem::transmute_copy(&code)).into()
        }
        unsafe extern "system" fn Hold<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fhold: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Hold(::core::mem::transmute_copy(&fhold)).into()
        }
        unsafe extern "system" fn HandoffDirect<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandoffDirect(::core::mem::transmute(&papplicationname)).into()
        }
        unsafe extern "system" fn HandoffIndirect<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandoffIndirect(::core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn Conference<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Conference(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn Transfer<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Transfer(::core::mem::transmute(&pcall), ::core::mem::transmute_copy(&fsync)).into()
        }
        unsafe extern "system" fn BlindTransfer<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BlindTransfer(::core::mem::transmute(&pdestaddress)).into()
        }
        unsafe extern "system" fn SwapHold<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SwapHold(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn ParkDirect<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparkaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ParkDirect(::core::mem::transmute(&pparkaddress)).into()
        }
        unsafe extern "system" fn ParkIndirect<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnondiraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ParkIndirect() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnondiraddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unpark<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unpark().into()
        }
        unsafe extern "system" fn SetQOS<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQOS(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&servicelevel)).into()
        }
        unsafe extern "system" fn Pickup<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgroupid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pickup(::core::mem::transmute(&pgroupid)).into()
        }
        unsafe extern "system" fn Dial<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Dial(::core::mem::transmute(&pdestaddress)).into()
        }
        unsafe extern "system" fn Finish<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finishmode: FINISH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&finishmode)).into()
        }
        unsafe extern "system" fn RemoveFromConference<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveFromConference().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Answer: Answer::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Hold: Hold::<Identity, Impl, OFFSET>,
            HandoffDirect: HandoffDirect::<Identity, Impl, OFFSET>,
            HandoffIndirect: HandoffIndirect::<Identity, Impl, OFFSET>,
            Conference: Conference::<Identity, Impl, OFFSET>,
            Transfer: Transfer::<Identity, Impl, OFFSET>,
            BlindTransfer: BlindTransfer::<Identity, Impl, OFFSET>,
            SwapHold: SwapHold::<Identity, Impl, OFFSET>,
            ParkDirect: ParkDirect::<Identity, Impl, OFFSET>,
            ParkIndirect: ParkIndirect::<Identity, Impl, OFFSET>,
            Unpark: Unpark::<Identity, Impl, OFFSET>,
            SetQOS: SetQOS::<Identity, Impl, OFFSET>,
            Pickup: Pickup::<Identity, Impl, OFFSET>,
            Dial: Dial::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            RemoveFromConference: RemoveFromConference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicCallControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITBasicCallControl2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITBasicCallControl_Impl {
    fn RequestTerminal(&self, bstrterminalclassguid: &super::super::Foundation::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
    fn SelectTerminalOnCall(&self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn UnselectTerminalOnCall(&self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITBasicCallControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl2_Impl, const OFFSET: isize>() -> ITBasicCallControl2_Vtbl {
        unsafe extern "system" fn RequestTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrterminalclassguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestTerminal(::core::mem::transmute(&bstrterminalclassguid), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectTerminalOnCall<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SelectTerminalOnCall(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminalOnCall<Identity: ::windows::core::IUnknownImpl, Impl: ITBasicCallControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnselectTerminalOnCall(::core::mem::transmute(&pterminal)).into()
        }
        Self {
            base: ITBasicCallControl_Vtbl::new::<Identity, Impl, OFFSET>(),
            RequestTerminal: RequestTerminal::<Identity, Impl, OFFSET>,
            SelectTerminalOnCall: SelectTerminalOnCall::<Identity, Impl, OFFSET>,
            UnselectTerminalOnCall: UnselectTerminalOnCall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITBasicCallControl2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITBasicCallControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallHub_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn EnumerateCalls(&self) -> ::windows::core::Result<IEnumCall>;
    fn Calls(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NumCalls(&self) -> ::windows::core::Result<i32>;
    fn State(&self) -> ::windows::core::Result<CALLHUB_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallHub_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHub_Impl, const OFFSET: isize>() -> ITCallHub_Vtbl {
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn EnumerateCalls<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcalls: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Calls() {
                ::core::result::Result::Ok(ok__) => {
                    *pcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumCalls<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumCalls() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHub_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut CALLHUB_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Clear: Clear::<Identity, Impl, OFFSET>,
            EnumerateCalls: EnumerateCalls::<Identity, Impl, OFFSET>,
            Calls: Calls::<Identity, Impl, OFFSET>,
            NumCalls: NumCalls::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallHub as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallHubEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Event(&self) -> ::windows::core::Result<CALLHUB_EVENT>;
    fn CallHub(&self) -> ::windows::core::Result<ITCallHub>;
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallHubEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHubEvent_Impl, const OFFSET: isize>() -> ITCallHubEvent_Vtbl {
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHubEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut CALLHUB_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHubEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallHub() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITCallHubEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Event: Event::<Identity, Impl, OFFSET>,
            CallHub: CallHub::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallHubEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&self) -> ::windows::core::Result<ITAddress>;
    fn CallState(&self) -> ::windows::core::Result<CALL_STATE>;
    fn Privilege(&self) -> ::windows::core::Result<CALL_PRIVILEGE>;
    fn CallHub(&self) -> ::windows::core::Result<ITCallHub>;
    fn CallInfoLong(&self, callinfolong: CALLINFO_LONG) -> ::windows::core::Result<i32>;
    fn SetCallInfoLong(&self, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::Result<()>;
    fn CallInfoString(&self, callinfostring: CALLINFO_STRING) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCallInfoString(&self, callinfostring: CALLINFO_STRING, pcallinfostring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetCallInfoBuffer2(&self, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows::core::Result<()>;
    fn ReleaseUserUserInfo(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>() -> ITCallInfo_Vtbl {
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallState<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallState() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivilege: *mut CALL_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Privilege() {
                ::core::result::Result::Ok(ok__) => {
                    *pprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallHub() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallInfoLong<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallInfoLong(::core::mem::transmute_copy(&callinfolong)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcallinfolongval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoLong<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCallInfoLong(::core::mem::transmute_copy(&callinfolong), ::core::mem::transmute_copy(&lcallinfolongval)).into()
        }
        unsafe extern "system" fn CallInfoString<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallInfoString(::core::mem::transmute_copy(&callinfostring)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfostring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoString<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, pcallinfostring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCallInfoString(::core::mem::transmute_copy(&callinfostring), ::core::mem::transmute(&pcallinfostring)).into()
        }
        unsafe extern "system" fn CallInfoBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfobuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn GetCallInfoBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCallInfoBuffer(::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppcallinfobuffer)).into()
        }
        unsafe extern "system" fn SetCallInfoBuffer2<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCallInfoBuffer2(::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pcallinfobuffer)).into()
        }
        unsafe extern "system" fn ReleaseUserUserInfo<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseUserUserInfo().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            CallState: CallState::<Identity, Impl, OFFSET>,
            Privilege: Privilege::<Identity, Impl, OFFSET>,
            CallHub: CallHub::<Identity, Impl, OFFSET>,
            CallInfoLong: CallInfoLong::<Identity, Impl, OFFSET>,
            SetCallInfoLong: SetCallInfoLong::<Identity, Impl, OFFSET>,
            CallInfoString: CallInfoString::<Identity, Impl, OFFSET>,
            SetCallInfoString: SetCallInfoString::<Identity, Impl, OFFSET>,
            CallInfoBuffer: CallInfoBuffer::<Identity, Impl, OFFSET>,
            SetCallInfoBuffer: SetCallInfoBuffer::<Identity, Impl, OFFSET>,
            GetCallInfoBuffer: GetCallInfoBuffer::<Identity, Impl, OFFSET>,
            SetCallInfoBuffer2: SetCallInfoBuffer2::<Identity, Impl, OFFSET>,
            ReleaseUserUserInfo: ReleaseUserUserInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfo2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITCallInfo_Impl {
    fn EventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows::core::Result<i16>;
    fn SetEventFilter(&self, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo2_Impl, const OFFSET: isize>() -> ITCallInfo2_Vtbl {
        unsafe extern "system" fn EventFilter<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *penable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent), ::core::mem::transmute_copy(&benable)).into()
        }
        Self {
            base: ITCallInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventFilter: EventFilter::<Identity, Impl, OFFSET>,
            SetEventFilter: SetEventFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfo2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITCallInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallInfoChangeEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn Cause(&self) -> ::windows::core::Result<CALLINFOCHANGE_CAUSE>;
    fn CallbackInstance(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallInfoChangeEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>() -> ITCallInfoChangeEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcic: *mut CALLINFOCHANGE_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows::core::IUnknownImpl, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallInfoChangeEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallMediaEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn Event(&self) -> ::windows::core::Result<CALL_MEDIA_EVENT>;
    fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Terminal(&self) -> ::windows::core::Result<ITTerminal>;
    fn Stream(&self) -> ::windows::core::Result<ITStream>;
    fn Cause(&self) -> ::windows::core::Result<CALL_MEDIA_EVENT_CAUSE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallMediaEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>() -> ITCallMediaEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallmediaevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerror = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: ::windows::core::IUnknownImpl, Impl: ITCallMediaEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcause = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallMediaEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallNotificationEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn Event(&self) -> ::windows::core::Result<CALL_NOTIFICATION_EVENT>;
    fn CallbackInstance(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallNotificationEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>() -> ITCallNotificationEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallnotificationevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows::core::IUnknownImpl, Impl: ITCallNotificationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallNotificationEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallStateEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn State(&self) -> ::windows::core::Result<CALL_STATE>;
    fn Cause(&self) -> ::windows::core::Result<CALL_STATE_EVENT_CAUSE>;
    fn CallbackInstance(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallStateEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallStateEvent_Impl, const OFFSET: isize>() -> ITCallStateEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pcallstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: ::windows::core::IUnknownImpl, Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcec: *mut CALL_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows::core::IUnknownImpl, Impl: ITCallStateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallStateEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCallingCard_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PermanentCardID(&self) -> ::windows::core::Result<i32>;
    fn NumberOfDigits(&self) -> ::windows::core::Result<i32>;
    fn Options(&self) -> ::windows::core::Result<i32>;
    fn CardName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SameAreaDialingRule(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LongDistanceDialingRule(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn InternationalDialingRule(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCallingCard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCard_Impl, const OFFSET: isize>() -> ITCallingCard_Vtbl {
        unsafe extern "system" fn PermanentCardID<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermanentCardID() {
                ::core::result::Result::Ok(ok__) => {
                    *plcardid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfDigits<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldigits: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberOfDigits() {
                ::core::result::Result::Ok(ok__) => {
                    *pldigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *ploptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardName<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcardname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CardName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcardname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SameAreaDialingRule<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SameAreaDialingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pprule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceDialingRule<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LongDistanceDialingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pprule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternationalDialingRule<Identity: ::windows::core::IUnknownImpl, Impl: ITCallingCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InternationalDialingRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pprule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PermanentCardID: PermanentCardID::<Identity, Impl, OFFSET>,
            NumberOfDigits: NumberOfDigits::<Identity, Impl, OFFSET>,
            Options: Options::<Identity, Impl, OFFSET>,
            CardName: CardName::<Identity, Impl, OFFSET>,
            SameAreaDialingRule: SameAreaDialingRule::<Identity, Impl, OFFSET>,
            LongDistanceDialingRule: LongDistanceDialingRule::<Identity, Impl, OFFSET>,
            InternationalDialingRule: InternationalDialingRule::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCallingCard as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection_Impl, const OFFSET: isize>() -> ITCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *lcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnewenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCollection2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITCollection_Impl {
    fn Add(&self, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&self, index: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCollection2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection2_Impl, const OFFSET: isize>() -> ITCollection2_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pvariant)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: ITCollection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&index)).into()
        }
        Self { base: ITCollection_Vtbl::new::<Identity, Impl, OFFSET>(), Add: Add::<Identity, Impl, OFFSET>, Remove: Remove::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCollection2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITCustomTone_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Frequency(&self) -> ::windows::core::Result<i32>;
    fn SetFrequency(&self, lfrequency: i32) -> ::windows::core::Result<()>;
    fn CadenceOn(&self) -> ::windows::core::Result<i32>;
    fn SetCadenceOn(&self, cadenceon: i32) -> ::windows::core::Result<()>;
    fn CadenceOff(&self) -> ::windows::core::Result<i32>;
    fn SetCadenceOff(&self, lcadenceoff: i32) -> ::windows::core::Result<()>;
    fn Volume(&self) -> ::windows::core::Result<i32>;
    fn SetVolume(&self, lvolume: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITCustomTone_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const OFFSET: isize>() -> ITCustomTone_Vtbl {
        unsafe extern "system" fn Frequency<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Frequency() {
                ::core::result::Result::Ok(ok__) => {
                    *plfrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfrequency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFrequency(::core::mem::transmute_copy(&lfrequency)).into()
        }
        unsafe extern "system" fn CadenceOn<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcadenceon: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CadenceOn() {
                ::core::result::Result::Ok(ok__) => {
                    *plcadenceon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOn<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cadenceon: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCadenceOn(::core::mem::transmute_copy(&cadenceon)).into()
        }
        unsafe extern "system" fn CadenceOff<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcadenceoff: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CadenceOff() {
                ::core::result::Result::Ok(ok__) => {
                    *plcadenceoff = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOff<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcadenceoff: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCadenceOff(::core::mem::transmute_copy(&lcadenceoff)).into()
        }
        unsafe extern "system" fn Volume<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows::core::IUnknownImpl, Impl: ITCustomTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Frequency: Frequency::<Identity, Impl, OFFSET>,
            SetFrequency: SetFrequency::<Identity, Impl, OFFSET>,
            CadenceOn: CadenceOn::<Identity, Impl, OFFSET>,
            SetCadenceOn: SetCadenceOn::<Identity, Impl, OFFSET>,
            CadenceOff: CadenceOff::<Identity, Impl, OFFSET>,
            SetCadenceOff: SetCadenceOff::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITCustomTone as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDetectTone_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn SetAppSpecific(&self, lappspecific: i32) -> ::windows::core::Result<()>;
    fn Duration(&self) -> ::windows::core::Result<i32>;
    fn SetDuration(&self, lduration: i32) -> ::windows::core::Result<()>;
    fn Frequency(&self, index: i32) -> ::windows::core::Result<i32>;
    fn SetFrequency(&self, index: i32, lfrequency: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDetectTone_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectTone_Impl, const OFFSET: isize>() -> ITDetectTone_Vtbl {
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAppSpecific(::core::mem::transmute_copy(&lappspecific)).into()
        }
        unsafe extern "system" fn Duration<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *plduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDuration(::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn Frequency<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Frequency(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *plfrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Identity: ::windows::core::IUnknownImpl, Impl: ITDetectTone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, lfrequency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFrequency(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&lfrequency)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            Frequency: Frequency::<Identity, Impl, OFFSET>,
            SetFrequency: SetFrequency::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDetectTone as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitDetectionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn Digit(&self) -> ::windows::core::Result<u8>;
    fn DigitMode(&self) -> ::windows::core::Result<i32>;
    fn TickCount(&self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitDetectionEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>() -> ITDigitDetectionEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digit<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pucdigit: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Digit() {
                ::core::result::Result::Ok(ok__) => {
                    *pucdigit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitMode<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigitmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DigitMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pdigitmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Digit: Digit::<Identity, Impl, OFFSET>,
            DigitMode: DigitMode::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitDetectionEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitGenerationEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn GenerationTermination(&self) -> ::windows::core::Result<i32>;
    fn TickCount(&self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitGenerationEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>() -> ITDigitGenerationEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerationTermination<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plgenerationtermination: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerationTermination() {
                ::core::result::Result::Ok(ok__) => {
                    *plgenerationtermination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitGenerationEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            GenerationTermination: GenerationTermination::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitGenerationEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDigitsGatheredEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn Digits(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GatherTermination(&self) -> ::windows::core::Result<TAPI_GATHERTERM>;
    fn TickCount(&self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDigitsGatheredEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>() -> ITDigitsGatheredEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digits<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdigits: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Digits() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdigits = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GatherTermination<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgathertermination: *mut TAPI_GATHERTERM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GatherTermination() {
                ::core::result::Result::Ok(ok__) => {
                    *pgathertermination = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows::core::IUnknownImpl, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Digits: Digits::<Identity, Impl, OFFSET>,
            GatherTermination: GatherTermination::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDigitsGatheredEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DirectoryType(&self) -> ::windows::core::Result<DIRECTORY_TYPE>;
    fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsDynamic(&self) -> ::windows::core::Result<i16>;
    fn DefaultObjectTTL(&self) -> ::windows::core::Result<i32>;
    fn SetDefaultObjectTTL(&self, ttl: i32) -> ::windows::core::Result<()>;
    fn EnableAutoRefresh(&self, fenable: i16) -> ::windows::core::Result<()>;
    fn Connect(&self, fsecure: i16) -> ::windows::core::Result<()>;
    fn Bind(&self, pdomainname: &super::super::Foundation::BSTR, pusername: &super::super::Foundation::BSTR, ppassword: &super::super::Foundation::BSTR, lflags: i32) -> ::windows::core::Result<()>;
    fn AddDirectoryObject(&self, pdirectoryobject: &::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn ModifyDirectoryObject(&self, pdirectoryobject: &::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn RefreshDirectoryObject(&self, pdirectoryobject: &::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn DeleteDirectoryObject(&self, pdirectoryobject: &::core::option::Option<ITDirectoryObject>) -> ::windows::core::Result<()>;
    fn DirectoryObjects(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDirectoryObjects(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IEnumDirectoryObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>() -> ITDirectory_Vtbl {
        unsafe extern "system" fn DirectoryType<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectorytype: *mut DIRECTORY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DirectoryType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirectorytype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDynamic<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfdynamic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDynamic() {
                ::core::result::Result::Ok(ok__) => {
                    *pfdynamic = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultObjectTTL<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultObjectTTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultObjectTTL<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ttl: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDefaultObjectTTL(::core::mem::transmute_copy(&ttl)).into()
        }
        unsafe extern "system" fn EnableAutoRefresh<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableAutoRefresh(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsecure: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&fsecure)).into()
        }
        unsafe extern "system" fn Bind<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Bind(::core::mem::transmute(&pdomainname), ::core::mem::transmute(&pusername), ::core::mem::transmute(&ppassword), ::core::mem::transmute_copy(&lflags)).into()
        }
        unsafe extern "system" fn AddDirectoryObject<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn ModifyDirectoryObject<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ModifyDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn RefreshDirectoryObject<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RefreshDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn DeleteDirectoryObject<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteDirectoryObject(::core::mem::transmute(&pdirectoryobject)).into()
        }
        unsafe extern "system" fn DirectoryObjects<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DirectoryObjects(::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDirectoryObjects<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenumobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateDirectoryObjects(::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DirectoryType: DirectoryType::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            IsDynamic: IsDynamic::<Identity, Impl, OFFSET>,
            DefaultObjectTTL: DefaultObjectTTL::<Identity, Impl, OFFSET>,
            SetDefaultObjectTTL: SetDefaultObjectTTL::<Identity, Impl, OFFSET>,
            EnableAutoRefresh: EnableAutoRefresh::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Bind: Bind::<Identity, Impl, OFFSET>,
            AddDirectoryObject: AddDirectoryObject::<Identity, Impl, OFFSET>,
            ModifyDirectoryObject: ModifyDirectoryObject::<Identity, Impl, OFFSET>,
            RefreshDirectoryObject: RefreshDirectoryObject::<Identity, Impl, OFFSET>,
            DeleteDirectoryObject: DeleteDirectoryObject::<Identity, Impl, OFFSET>,
            DirectoryObjects: DirectoryObjects::<Identity, Impl, OFFSET>,
            EnumerateDirectoryObjects: EnumerateDirectoryObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectory as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObject_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ObjectType(&self) -> ::windows::core::Result<DIRECTORY_OBJECT_TYPE>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DialableAddrs(&self, dwaddresstype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDialableAddrs(&self, dwaddresstype: u32) -> ::windows::core::Result<IEnumDialableAddrs>;
    fn SecurityDescriptor(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(&self, psecdes: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObject_Impl, const OFFSET: isize>() -> ITDirectoryObject_Vtbl {
        unsafe extern "system" fn ObjectType<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    *pobjecttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute(&pname)).into()
        }
        unsafe extern "system" fn DialableAddrs<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddresstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DialableAddrs(::core::mem::transmute_copy(&dwaddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDialableAddrs<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddresstype: u32, ppenumdialableaddrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateDialableAddrs(::core::mem::transmute_copy(&dwaddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdialableaddrs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecdes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SecurityDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecdes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psecdes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecurityDescriptor(::core::mem::transmute(&psecdes)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ObjectType: ObjectType::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            DialableAddrs: DialableAddrs::<Identity, Impl, OFFSET>,
            EnumerateDialableAddrs: EnumerateDialableAddrs::<Identity, Impl, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObject as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectConference_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Protocol(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Originator(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetOriginator(&self, poriginator: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AdvertisingScope(&self) -> ::windows::core::Result<RND_ADVERTISING_SCOPE>;
    fn SetAdvertisingScope(&self, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::Result<()>;
    fn Url(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetUrl(&self, purl: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, pdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsEncrypted(&self) -> ::windows::core::Result<i16>;
    fn SetIsEncrypted(&self, fencrypted: i16) -> ::windows::core::Result<()>;
    fn StartTime(&self) -> ::windows::core::Result<f64>;
    fn SetStartTime(&self, date: f64) -> ::windows::core::Result<()>;
    fn StopTime(&self) -> ::windows::core::Result<f64>;
    fn SetStopTime(&self, date: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectConference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>() -> ITDirectoryObjectConference_Vtbl {
        unsafe extern "system" fn Protocol<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *ppprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Originator<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporiginator: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Originator() {
                ::core::result::Result::Ok(ok__) => {
                    *pporiginator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginator<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poriginator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOriginator(::core::mem::transmute(&poriginator)).into()
        }
        unsafe extern "system" fn AdvertisingScope<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdvertisingScope() {
                ::core::result::Result::Ok(ok__) => {
                    *padvertisingscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisingScope<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAdvertisingScope(::core::mem::transmute_copy(&advertisingscope)).into()
        }
        unsafe extern "system" fn Url<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Url() {
                ::core::result::Result::Ok(ok__) => {
                    *ppurl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUrl(::core::mem::transmute(&purl)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn IsEncrypted<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfencrypted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEncrypted() {
                ::core::result::Result::Ok(ok__) => {
                    *pfencrypted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEncrypted<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fencrypted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsEncrypted(::core::mem::transmute_copy(&fencrypted)).into()
        }
        unsafe extern "system" fn StartTime<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStartTime(::core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn StopTime<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StopTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pdate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopTime<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectConference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStopTime(::core::mem::transmute_copy(&date)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            Originator: Originator::<Identity, Impl, OFFSET>,
            SetOriginator: SetOriginator::<Identity, Impl, OFFSET>,
            AdvertisingScope: AdvertisingScope::<Identity, Impl, OFFSET>,
            SetAdvertisingScope: SetAdvertisingScope::<Identity, Impl, OFFSET>,
            Url: Url::<Identity, Impl, OFFSET>,
            SetUrl: SetUrl::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            IsEncrypted: IsEncrypted::<Identity, Impl, OFFSET>,
            SetIsEncrypted: SetIsEncrypted::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            StopTime: StopTime::<Identity, Impl, OFFSET>,
            SetStopTime: SetStopTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObjectConference as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDirectoryObjectUser_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IPPhonePrimary(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetIPPhonePrimary(&self, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDirectoryObjectUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectUser_Impl, const OFFSET: isize>() -> ITDirectoryObjectUser_Vtbl {
        unsafe extern "system" fn IPPhonePrimary<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IPPhonePrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPPhonePrimary<Identity: ::windows::core::IUnknownImpl, Impl: ITDirectoryObjectUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIPPhonePrimary(::core::mem::transmute(&pname)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IPPhonePrimary: IPPhonePrimary::<Identity, Impl, OFFSET>,
            SetIPPhonePrimary: SetIPPhonePrimary::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDirectoryObjectUser as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITDispatchMapper_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn QueryDispatchInterface(&self, piid: &super::super::Foundation::BSTR, pinterfacetomap: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITDispatchMapper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITDispatchMapper_Impl, const OFFSET: isize>() -> ITDispatchMapper_Vtbl {
        unsafe extern "system" fn QueryDispatchInterface<Identity: ::windows::core::IUnknownImpl, Impl: ITDispatchMapper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pinterfacetomap: ::windows::core::RawPtr, ppreturnedinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryDispatchInterface(::core::mem::transmute(&piid), ::core::mem::transmute(&pinterfacetomap)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppreturnedinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            QueryDispatchInterface: QueryDispatchInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITDispatchMapper as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITFileTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&self) -> ::windows::core::Result<ITTerminal>;
    fn Track(&self) -> ::windows::core::Result<ITFileTrack>;
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn State(&self) -> ::windows::core::Result<TERMINAL_MEDIA_STATE>;
    fn Cause(&self) -> ::windows::core::Result<FT_STATE_EVENT_CAUSE>;
    fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITFileTerminalEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>() -> ITFileTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrackterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Track() {
                ::core::result::Result::Ok(ok__) => {
                    *pptrackterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcause: *mut FT_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Cause() {
                ::core::result::Result::Ok(ok__) => {
                    *pcause = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Track: Track::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITFileTerminalEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITFileTrack_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Format(&self) -> ::windows::core::Result<*mut super::super::Media::DirectShow::AM_MEDIA_TYPE>;
    fn SetFormat(&self, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
    fn ControllingTerminal(&self) -> ::windows::core::Result<ITTerminal>;
    fn AudioFormatForScripting(&self) -> ::windows::core::Result<ITScriptableAudioFormat>;
    fn SetAudioFormatForScripting(&self, paudioformat: &::core::option::Option<ITScriptableAudioFormat>) -> ::windows::core::Result<()>;
    fn EmptyAudioFormatForScripting(&self) -> ::windows::core::Result<ITScriptableAudioFormat>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITFileTrack_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrack_Impl, const OFFSET: isize>() -> ITFileTrack_Vtbl {
        unsafe extern "system" fn Format<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmt = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&pmt)).into()
        }
        unsafe extern "system" fn ControllingTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontrollingterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ControllingTerminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontrollingterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatForScripting<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AudioFormatForScripting() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaudioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioFormatForScripting<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudioformat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAudioFormatForScripting(::core::mem::transmute(&paudioformat)).into()
        }
        unsafe extern "system" fn EmptyAudioFormatForScripting<Identity: ::windows::core::IUnknownImpl, Impl: ITFileTrack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EmptyAudioFormatForScripting() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaudioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            ControllingTerminal: ControllingTerminal::<Identity, Impl, OFFSET>,
            AudioFormatForScripting: AudioFormatForScripting::<Identity, Impl, OFFSET>,
            SetAudioFormatForScripting: SetAudioFormatForScripting::<Identity, Impl, OFFSET>,
            EmptyAudioFormatForScripting: EmptyAudioFormatForScripting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITFileTrack as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITForwardInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetNumRingsNoAnswer(&self, lnumrings: i32) -> ::windows::core::Result<()>;
    fn NumRingsNoAnswer(&self) -> ::windows::core::Result<i32>;
    fn SetForwardType(&self, forwardtype: i32, pdestaddress: &super::super::Foundation::BSTR, pcalleraddress: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ForwardTypeDestination(&self, forwardtype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ForwardTypeCaller(&self, forwardtype: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetForwardType(&self, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITForwardInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation_Impl, const OFFSET: isize>() -> ITForwardInformation_Vtbl {
        unsafe extern "system" fn SetNumRingsNoAnswer<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumrings: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumRingsNoAnswer(::core::mem::transmute_copy(&lnumrings)).into()
        }
        unsafe extern "system" fn NumRingsNoAnswer<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plnumrings: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumRingsNoAnswer() {
                ::core::result::Result::Ok(ok__) => {
                    *plnumrings = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForwardType<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForwardType(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute(&pdestaddress), ::core::mem::transmute(&pcalleraddress)).into()
        }
        unsafe extern "system" fn ForwardTypeDestination<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForwardTypeDestination(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeCaller<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForwardTypeCaller(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcalleraddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForwardType<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForwardType(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&ppcalleraddress)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetNumRingsNoAnswer: SetNumRingsNoAnswer::<Identity, Impl, OFFSET>,
            NumRingsNoAnswer: NumRingsNoAnswer::<Identity, Impl, OFFSET>,
            SetForwardType: SetForwardType::<Identity, Impl, OFFSET>,
            ForwardTypeDestination: ForwardTypeDestination::<Identity, Impl, OFFSET>,
            ForwardTypeCaller: ForwardTypeCaller::<Identity, Impl, OFFSET>,
            GetForwardType: GetForwardType::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITForwardInformation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITForwardInformation2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITForwardInformation_Impl {
    fn SetForwardType2(&self, forwardtype: i32, pdestaddress: &super::super::Foundation::BSTR, destaddresstype: i32, pcalleraddress: &super::super::Foundation::BSTR, calleraddresstype: i32) -> ::windows::core::Result<()>;
    fn GetForwardType2(&self, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut super::super::Foundation::BSTR, pcalleraddresstype: *mut i32) -> ::windows::core::Result<()>;
    fn ForwardTypeDestinationAddressType(&self, forwardtype: i32) -> ::windows::core::Result<i32>;
    fn ForwardTypeCallerAddressType(&self, forwardtype: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITForwardInformation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation2_Impl, const OFFSET: isize>() -> ITForwardInformation2_Vtbl {
        unsafe extern "system" fn SetForwardType2<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destaddresstype: i32, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, calleraddresstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForwardType2(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute(&pdestaddress), ::core::mem::transmute_copy(&destaddresstype), ::core::mem::transmute(&pcalleraddress), ::core::mem::transmute_copy(&calleraddresstype)).into()
        }
        unsafe extern "system" fn GetForwardType2<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut super::super::Foundation::BSTR, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetForwardType2(::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&pdestaddresstype), ::core::mem::transmute_copy(&ppcalleraddress), ::core::mem::transmute_copy(&pcalleraddresstype)).into()
        }
        unsafe extern "system" fn ForwardTypeDestinationAddressType<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForwardTypeDestinationAddressType(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdestaddresstype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeCallerAddressType<Identity: ::windows::core::IUnknownImpl, Impl: ITForwardInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ForwardTypeCallerAddressType(::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcalleraddresstype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITForwardInformation_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetForwardType2: SetForwardType2::<Identity, Impl, OFFSET>,
            GetForwardType2: GetForwardType2::<Identity, Impl, OFFSET>,
            ForwardTypeDestinationAddressType: ForwardTypeDestinationAddressType::<Identity, Impl, OFFSET>,
            ForwardTypeCallerAddressType: ForwardTypeCallerAddressType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITForwardInformation2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITForwardInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITILSConfig_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Port(&self) -> ::windows::core::Result<i32>;
    fn SetPort(&self, port: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITILSConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITILSConfig_Impl, const OFFSET: isize>() -> ITILSConfig_Vtbl {
        unsafe extern "system" fn Port<Identity: ::windows::core::IUnknownImpl, Impl: ITILSConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Port() {
                ::core::result::Result::Ok(ok__) => {
                    *pport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Identity: ::windows::core::IUnknownImpl, Impl: ITILSConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, port: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPort(::core::mem::transmute_copy(&port)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Port: Port::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITILSConfig as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITLegacyAddressMediaControl_Impl: Sized {
    fn GetID(&self, pdeviceclass: &super::super::Foundation::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()>;
    fn GetDevConfig(&self, pdeviceclass: &super::super::Foundation::BSTR, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetDevConfig(&self, pdeviceclass: &super::super::Foundation::BSTR, dwsize: u32, pdeviceconfig: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITLegacyAddressMediaControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>() -> ITLegacyAddressMediaControl_Vtbl {
        unsafe extern "system" fn GetID<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetID(::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn GetDevConfig<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDevConfig(::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceconfig)).into()
        }
        unsafe extern "system" fn SetDevConfig<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsize: u32, pdeviceconfig: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDevConfig(::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdeviceconfig)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetDevConfig: GetDevConfig::<Identity, Impl, OFFSET>,
            SetDevConfig: SetDevConfig::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyAddressMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITLegacyAddressMediaControl2_Impl: Sized + ITLegacyAddressMediaControl_Impl {
    fn ConfigDialog(&self, hwndowner: super::super::Foundation::HWND, pdeviceclass: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ConfigDialogEdit(&self, hwndowner: super::super::Foundation::HWND, pdeviceclass: &super::super::Foundation::BSTR, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ITLegacyAddressMediaControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: isize>() -> ITLegacyAddressMediaControl2_Vtbl {
        unsafe extern "system" fn ConfigDialog<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConfigDialog(::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute(&pdeviceclass)).into()
        }
        unsafe extern "system" fn ConfigDialogEdit<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConfigDialogEdit(::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&dwsizein), ::core::mem::transmute_copy(&pdeviceconfigin), ::core::mem::transmute_copy(&pdwsizeout), ::core::mem::transmute_copy(&ppdeviceconfigout)).into()
        }
        Self {
            base: ITLegacyAddressMediaControl_Vtbl::new::<Identity, Impl, OFFSET>(),
            ConfigDialog: ConfigDialog::<Identity, Impl, OFFSET>,
            ConfigDialogEdit: ConfigDialogEdit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyAddressMediaControl2 as ::windows::core::Interface>::IID || iid == &<ITLegacyAddressMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyCallMediaControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DetectDigits(&self, digitmode: i32) -> ::windows::core::Result<()>;
    fn GenerateDigits(&self, pdigits: &super::super::Foundation::BSTR, digitmode: i32) -> ::windows::core::Result<()>;
    fn GetID(&self, pdeviceclass: &super::super::Foundation::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()>;
    fn SetMediaType(&self, lmediatype: i32) -> ::windows::core::Result<()>;
    fn MonitorMedia(&self, lmediatype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyCallMediaControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>() -> ITLegacyCallMediaControl_Vtbl {
        unsafe extern "system" fn DetectDigits<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DetectDigits(::core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GenerateDigits<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateDigits(::core::mem::transmute(&pdigits), ::core::mem::transmute_copy(&digitmode)).into()
        }
        unsafe extern "system" fn GetID<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetID(::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)).into()
        }
        unsafe extern "system" fn SetMediaType<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMediaType(::core::mem::transmute_copy(&lmediatype)).into()
        }
        unsafe extern "system" fn MonitorMedia<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MonitorMedia(::core::mem::transmute_copy(&lmediatype)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DetectDigits: DetectDigits::<Identity, Impl, OFFSET>,
            GenerateDigits: GenerateDigits::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
            SetMediaType: SetMediaType::<Identity, Impl, OFFSET>,
            MonitorMedia: MonitorMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyCallMediaControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyCallMediaControl2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITLegacyCallMediaControl_Impl {
    fn GenerateDigits2(&self, pdigits: &super::super::Foundation::BSTR, digitmode: i32, lduration: i32) -> ::windows::core::Result<()>;
    fn GatherDigits(&self, digitmode: i32, lnumdigits: i32, pterminationdigits: &super::super::Foundation::BSTR, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::Result<()>;
    fn DetectTones(&self, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::Result<()>;
    fn DetectTonesByCollection(&self, pdetecttonecollection: &::core::option::Option<ITCollection2>) -> ::windows::core::Result<()>;
    fn GenerateTone(&self, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::Result<()>;
    fn GenerateCustomTones(&self, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::Result<()>;
    fn GenerateCustomTonesByCollection(&self, pcustomtonecollection: &::core::option::Option<ITCollection2>, lduration: i32) -> ::windows::core::Result<()>;
    fn CreateDetectToneObject(&self) -> ::windows::core::Result<ITDetectTone>;
    fn CreateCustomToneObject(&self) -> ::windows::core::Result<ITCustomTone>;
    fn GetIDAsVariant(&self, bstrdeviceclass: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyCallMediaControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>() -> ITLegacyCallMediaControl2_Vtbl {
        unsafe extern "system" fn GenerateDigits2<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateDigits2(::core::mem::transmute(&pdigits), ::core::mem::transmute_copy(&digitmode), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GatherDigits<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, digitmode: i32, lnumdigits: i32, pterminationdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GatherDigits(::core::mem::transmute_copy(&digitmode), ::core::mem::transmute_copy(&lnumdigits), ::core::mem::transmute(&pterminationdigits), ::core::mem::transmute_copy(&lfirstdigittimeout), ::core::mem::transmute_copy(&linterdigittimeout)).into()
        }
        unsafe extern "system" fn DetectTones<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DetectTones(::core::mem::transmute_copy(&ptonelist), ::core::mem::transmute_copy(&lnumtones)).into()
        }
        unsafe extern "system" fn DetectTonesByCollection<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdetecttonecollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DetectTonesByCollection(::core::mem::transmute(&pdetecttonecollection)).into()
        }
        unsafe extern "system" fn GenerateTone<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateTone(::core::mem::transmute_copy(&tonemode), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTones<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateCustomTones(::core::mem::transmute_copy(&ptonelist), ::core::mem::transmute_copy(&lnumtones), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn GenerateCustomTonesByCollection<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustomtonecollection: ::windows::core::RawPtr, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateCustomTonesByCollection(::core::mem::transmute(&pcustomtonecollection), ::core::mem::transmute_copy(&lduration)).into()
        }
        unsafe extern "system" fn CreateDetectToneObject<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdetecttone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDetectToneObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdetecttone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomToneObject<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcustomtone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateCustomToneObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcustomtone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDAsVariant<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardeviceid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetIDAsVariant(::core::mem::transmute(&bstrdeviceclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvardeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITLegacyCallMediaControl_Vtbl::new::<Identity, Impl, OFFSET>(),
            GenerateDigits2: GenerateDigits2::<Identity, Impl, OFFSET>,
            GatherDigits: GatherDigits::<Identity, Impl, OFFSET>,
            DetectTones: DetectTones::<Identity, Impl, OFFSET>,
            DetectTonesByCollection: DetectTonesByCollection::<Identity, Impl, OFFSET>,
            GenerateTone: GenerateTone::<Identity, Impl, OFFSET>,
            GenerateCustomTones: GenerateCustomTones::<Identity, Impl, OFFSET>,
            GenerateCustomTonesByCollection: GenerateCustomTonesByCollection::<Identity, Impl, OFFSET>,
            CreateDetectToneObject: CreateDetectToneObject::<Identity, Impl, OFFSET>,
            CreateCustomToneObject: CreateCustomToneObject::<Identity, Impl, OFFSET>,
            GetIDAsVariant: GetIDAsVariant::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyCallMediaControl2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITLegacyCallMediaControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLegacyWaveSupport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsFullDuplex(&self) -> ::windows::core::Result<FULLDUPLEX_SUPPORT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLegacyWaveSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyWaveSupport_Impl, const OFFSET: isize>() -> ITLegacyWaveSupport_Vtbl {
        unsafe extern "system" fn IsFullDuplex<Identity: ::windows::core::IUnknownImpl, Impl: ITLegacyWaveSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psupport: *mut FULLDUPLEX_SUPPORT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFullDuplex() {
                ::core::result::Result::Ok(ok__) => {
                    *psupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), IsFullDuplex: IsFullDuplex::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLegacyWaveSupport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITLocationInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn PermanentLocationID(&self) -> ::windows::core::Result<i32>;
    fn CountryCode(&self) -> ::windows::core::Result<i32>;
    fn CountryID(&self) -> ::windows::core::Result<i32>;
    fn Options(&self) -> ::windows::core::Result<i32>;
    fn PreferredCardID(&self) -> ::windows::core::Result<i32>;
    fn LocationName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CityCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LocalAccessCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn LongDistanceAccessCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TollPrefixList(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CancelCallWaitingCode(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITLocationInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>() -> ITLocationInfo_Vtbl {
        unsafe extern "system" fn PermanentLocationID<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pllocationid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PermanentLocationID() {
                ::core::result::Result::Ok(ok__) => {
                    *pllocationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryCode<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *plcountrycode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryID<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcountryid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CountryID() {
                ::core::result::Result::Ok(ok__) => {
                    *plcountryid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *ploptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredCardID<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PreferredCardID() {
                ::core::result::Result::Ok(ok__) => {
                    *plcardid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationName<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplocationname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocationName() {
                ::core::result::Result::Ok(ok__) => {
                    *pplocationname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CityCode<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CityCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAccessCode<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalAccessCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceAccessCode<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LongDistanceAccessCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TollPrefixList<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptolllist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TollPrefixList() {
                ::core::result::Result::Ok(ok__) => {
                    *pptolllist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelCallWaitingCode<Identity: ::windows::core::IUnknownImpl, Impl: ITLocationInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CancelCallWaitingCode() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PermanentLocationID: PermanentLocationID::<Identity, Impl, OFFSET>,
            CountryCode: CountryCode::<Identity, Impl, OFFSET>,
            CountryID: CountryID::<Identity, Impl, OFFSET>,
            Options: Options::<Identity, Impl, OFFSET>,
            PreferredCardID: PreferredCardID::<Identity, Impl, OFFSET>,
            LocationName: LocationName::<Identity, Impl, OFFSET>,
            CityCode: CityCode::<Identity, Impl, OFFSET>,
            LocalAccessCode: LocalAccessCode::<Identity, Impl, OFFSET>,
            LongDistanceAccessCode: LongDistanceAccessCode::<Identity, Impl, OFFSET>,
            TollPrefixList: TollPrefixList::<Identity, Impl, OFFSET>,
            CancelCallWaitingCode: CancelCallWaitingCode::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITLocationInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait ITMSPAddress_Impl: Sized {
    fn Initialize(&self, hevent: *const i32) -> ::windows::core::Result<()>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
    fn CreateMSPCall(&self, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ShutdownMSPCall(&self, pstreamcontrol: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ReceiveTSPData(&self, pmspcall: &::core::option::Option<::windows::core::IUnknown>, pbuffer: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn GetEvent(&self, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::Result<()>;
}
impl ITMSPAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddress_Impl, const OFFSET: isize>() -> ITMSPAddress_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&hevent)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn CreateMSPCall<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut ::core::ffi::c_void, ppstreamcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateMSPCall(::core::mem::transmute_copy(&hcall), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&dwmediatype), ::core::mem::transmute(&pouterunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstreamcontrol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownMSPCall<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstreamcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShutdownMSPCall(::core::mem::transmute(&pstreamcontrol)).into()
        }
        unsafe extern "system" fn ReceiveTSPData<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmspcall: *mut ::core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReceiveTSPData(::core::mem::transmute(&pmspcall), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetEvent<Identity: ::windows::core::IUnknownImpl, Impl: ITMSPAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEvent(::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&peventbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            CreateMSPCall: CreateMSPCall::<Identity, Impl, OFFSET>,
            ShutdownMSPCall: ShutdownMSPCall::<Identity, Impl, OFFSET>,
            ReceiveTSPData: ReceiveTSPData::<Identity, Impl, OFFSET>,
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMSPAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn MediaState(&self) -> ::windows::core::Result<TERMINAL_MEDIA_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaControl_Impl, const OFFSET: isize>() -> ITMediaControl_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn MediaState<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaState() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminalmediastate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            MediaState: MediaState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaPlayback_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetPlayList(&self, playlistvariant: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PlayList(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaPlayback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaPlayback_Impl, const OFFSET: isize>() -> ITMediaPlayback_Vtbl {
        unsafe extern "system" fn SetPlayList<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaPlayback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, playlistvariant: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlayList(::core::mem::transmute(&playlistvariant)).into()
        }
        unsafe extern "system" fn PlayList<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaPlayback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplaylistvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PlayList() {
                ::core::result::Result::Ok(ok__) => {
                    *pplaylistvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPlayList: SetPlayList::<Identity, Impl, OFFSET>,
            PlayList: PlayList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaPlayback as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaRecord_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetFileName(&self, bstrfilename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FileName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaRecord_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaRecord_Impl, const OFFSET: isize>() -> ITMediaRecord_Vtbl {
        unsafe extern "system" fn SetFileName<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFileName(::core::mem::transmute(&bstrfilename)).into()
        }
        unsafe extern "system" fn FileName<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaRecord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FileName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfilename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetFileName: SetFileName::<Identity, Impl, OFFSET>,
            FileName: FileName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaRecord as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMediaSupport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaTypes(&self) -> ::windows::core::Result<i32>;
    fn QueryMediaType(&self, lmediatype: i32) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMediaSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaSupport_Impl, const OFFSET: isize>() -> ITMediaSupport_Vtbl {
        unsafe extern "system" fn MediaTypes<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMediaType<Identity: ::windows::core::IUnknownImpl, Impl: ITMediaSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, pfsupport: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryMediaType(::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MediaTypes: MediaTypes::<Identity, Impl, OFFSET>,
            QueryMediaType: QueryMediaType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMediaSupport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITMultiTrackTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TrackTerminals(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateTrackTerminals(&self) -> ::windows::core::Result<IEnumTerminal>;
    fn CreateTrackTerminal(&self, mediatype: i32, terminaldirection: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
    fn MediaTypesInUse(&self) -> ::windows::core::Result<i32>;
    fn DirectionsInUse(&self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
    fn RemoveTrackTerminal(&self, ptrackterminaltoremove: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITMultiTrackTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>() -> ITMultiTrackTerminal_Vtbl {
        unsafe extern "system" fn TrackTerminals<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrackTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTrackTerminals<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateTrackTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrackTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTrackTerminal(::core::mem::transmute_copy(&mediatype), ::core::mem::transmute_copy(&terminaldirection)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypesInUse<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatypesinuse: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaTypesInUse() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatypesinuse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectionsInUse<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pldirectionsinused: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DirectionsInUse() {
                ::core::result::Result::Ok(ok__) => {
                    *pldirectionsinused = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrackTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITMultiTrackTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptrackterminaltoremove: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveTrackTerminal(::core::mem::transmute(&ptrackterminaltoremove)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TrackTerminals: TrackTerminals::<Identity, Impl, OFFSET>,
            EnumerateTrackTerminals: EnumerateTrackTerminals::<Identity, Impl, OFFSET>,
            CreateTrackTerminal: CreateTrackTerminal::<Identity, Impl, OFFSET>,
            MediaTypesInUse: MediaTypesInUse::<Identity, Impl, OFFSET>,
            DirectionsInUse: DirectionsInUse::<Identity, Impl, OFFSET>,
            RemoveTrackTerminal: RemoveTrackTerminal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITMultiTrackTerminal as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhone_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Open(&self, privilege: PHONE_PRIVILEGE) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Addresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateAddresses(&self) -> ::windows::core::Result<IEnumAddress>;
    fn PhoneCapsLong(&self, pclcap: PHONECAPS_LONG) -> ::windows::core::Result<i32>;
    fn PhoneCapsString(&self, pcscap: PHONECAPS_STRING) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Terminals(&self, paddress: &::core::option::Option<ITAddress>) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateTerminals(&self, paddress: &::core::option::Option<ITAddress>) -> ::windows::core::Result<IEnumTerminal>;
    fn ButtonMode(&self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_MODE>;
    fn SetButtonMode(&self, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::Result<()>;
    fn ButtonFunction(&self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_FUNCTION>;
    fn SetButtonFunction(&self, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::Result<()>;
    fn ButtonText(&self, lbuttonid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetButtonText(&self, lbuttonid: i32, bstrbuttontext: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ButtonState(&self, lbuttonid: i32) -> ::windows::core::Result<PHONE_BUTTON_STATE>;
    fn HookSwitchState(&self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE) -> ::windows::core::Result<PHONE_HOOK_SWITCH_STATE>;
    fn SetHookSwitchState(&self, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows::core::Result<()>;
    fn SetRingMode(&self, lringmode: i32) -> ::windows::core::Result<()>;
    fn RingMode(&self) -> ::windows::core::Result<i32>;
    fn SetRingVolume(&self, lringvolume: i32) -> ::windows::core::Result<()>;
    fn RingVolume(&self) -> ::windows::core::Result<i32>;
    fn Privilege(&self) -> ::windows::core::Result<PHONE_PRIVILEGE>;
    fn GetPhoneCapsBuffer(&self, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows::core::Result<()>;
    fn PhoneCapsBuffer(&self, pcbcaps: PHONECAPS_BUFFER) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn LampMode(&self, llampid: i32) -> ::windows::core::Result<PHONE_LAMP_MODE>;
    fn SetLampMode(&self, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows::core::Result<()>;
    fn Display(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDisplay(&self, lrow: i32, lcolumn: i32, bstrdisplay: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PreferredAddresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePreferredAddresses(&self) -> ::windows::core::Result<IEnumAddress>;
    fn DeviceSpecific(&self, pparams: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn DeviceSpecificVariant(&self, vardevspecificbytearray: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn NegotiateExtVersion(&self, llowversion: i32, lhighversion: i32) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhone_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>() -> ITPhone_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privilege: PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&privilege)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Addresses<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *paddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneCapsLong<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhoneCapsLong(::core::mem::transmute_copy(&pclcap)) {
                ::core::result::Result::Ok(ok__) => {
                    *plcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneCapsString<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcscap: PHONECAPS_STRING, ppcapability: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhoneCapsString(::core::mem::transmute_copy(&pcscap)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcapability = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Terminals(::core::mem::transmute(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *pterminals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateTerminals(::core::mem::transmute(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonMode<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ButtonMode(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbuttonmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonMode<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetButtonMode(::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&buttonmode)).into()
        }
        unsafe extern "system" fn ButtonFunction<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ButtonFunction(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbuttonfunction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonFunction<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetButtonFunction(::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&buttonfunction)).into()
        }
        unsafe extern "system" fn ButtonText<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, ppbuttontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ButtonText(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuttontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonText<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, bstrbuttontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetButtonText(::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute(&bstrbuttontext)).into()
        }
        unsafe extern "system" fn ButtonState<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ButtonState(::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbuttonstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HookSwitchState(::core::mem::transmute_copy(&hookswitchdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *phookswitchstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHookSwitchState<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHookSwitchState(::core::mem::transmute_copy(&hookswitchdevice), ::core::mem::transmute_copy(&hookswitchstate)).into()
        }
        unsafe extern "system" fn SetRingMode<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRingMode(::core::mem::transmute_copy(&lringmode)).into()
        }
        unsafe extern "system" fn RingMode<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plringmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingVolume<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lringvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRingVolume(::core::mem::transmute_copy(&lringvolume)).into()
        }
        unsafe extern "system" fn RingVolume<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RingVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *plringvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprivilege: *mut PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Privilege() {
                ::core::result::Result::Ok(ok__) => {
                    *pprivilege = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneCapsBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPhoneCapsBuffer(::core::mem::transmute_copy(&pcbcaps), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppphonecapsbuffer)).into()
        }
        unsafe extern "system" fn PhoneCapsBuffer<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PhoneCapsBuffer(::core::mem::transmute_copy(&pcbcaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LampMode<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LampMode(::core::mem::transmute_copy(&llampid)) {
                ::core::result::Result::Ok(ok__) => {
                    *plampmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLampMode<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLampMode(::core::mem::transmute_copy(&llampid), ::core::mem::transmute_copy(&lampmode)).into()
        }
        unsafe extern "system" fn Display<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdisplay = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplay<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lrow: i32, lcolumn: i32, bstrdisplay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplay(::core::mem::transmute_copy(&lrow), ::core::mem::transmute_copy(&lcolumn), ::core::mem::transmute(&bstrdisplay)).into()
        }
        unsafe extern "system" fn PreferredAddresses<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PreferredAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *paddresses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredAddresses<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumeratePreferredAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSpecific<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceSpecific(::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn DeviceSpecificVariant<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeviceSpecificVariant(::core::mem::transmute(&vardevspecificbytearray)).into()
        }
        unsafe extern "system" fn NegotiateExtVersion<Identity: ::windows::core::IUnknownImpl, Impl: ITPhone_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NegotiateExtVersion(::core::mem::transmute_copy(&llowversion), ::core::mem::transmute_copy(&lhighversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *plextversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Addresses: Addresses::<Identity, Impl, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, Impl, OFFSET>,
            PhoneCapsLong: PhoneCapsLong::<Identity, Impl, OFFSET>,
            PhoneCapsString: PhoneCapsString::<Identity, Impl, OFFSET>,
            Terminals: Terminals::<Identity, Impl, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, Impl, OFFSET>,
            ButtonMode: ButtonMode::<Identity, Impl, OFFSET>,
            SetButtonMode: SetButtonMode::<Identity, Impl, OFFSET>,
            ButtonFunction: ButtonFunction::<Identity, Impl, OFFSET>,
            SetButtonFunction: SetButtonFunction::<Identity, Impl, OFFSET>,
            ButtonText: ButtonText::<Identity, Impl, OFFSET>,
            SetButtonText: SetButtonText::<Identity, Impl, OFFSET>,
            ButtonState: ButtonState::<Identity, Impl, OFFSET>,
            HookSwitchState: HookSwitchState::<Identity, Impl, OFFSET>,
            SetHookSwitchState: SetHookSwitchState::<Identity, Impl, OFFSET>,
            SetRingMode: SetRingMode::<Identity, Impl, OFFSET>,
            RingMode: RingMode::<Identity, Impl, OFFSET>,
            SetRingVolume: SetRingVolume::<Identity, Impl, OFFSET>,
            RingVolume: RingVolume::<Identity, Impl, OFFSET>,
            Privilege: Privilege::<Identity, Impl, OFFSET>,
            GetPhoneCapsBuffer: GetPhoneCapsBuffer::<Identity, Impl, OFFSET>,
            PhoneCapsBuffer: PhoneCapsBuffer::<Identity, Impl, OFFSET>,
            LampMode: LampMode::<Identity, Impl, OFFSET>,
            SetLampMode: SetLampMode::<Identity, Impl, OFFSET>,
            Display: Display::<Identity, Impl, OFFSET>,
            SetDisplay: SetDisplay::<Identity, Impl, OFFSET>,
            PreferredAddresses: PreferredAddresses::<Identity, Impl, OFFSET>,
            EnumeratePreferredAddresses: EnumeratePreferredAddresses::<Identity, Impl, OFFSET>,
            DeviceSpecific: DeviceSpecific::<Identity, Impl, OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Identity, Impl, OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhone as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneDeviceSpecificEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Phone(&self) -> ::windows::core::Result<ITPhone>;
    fn lParam1(&self) -> ::windows::core::Result<i32>;
    fn lParam2(&self) -> ::windows::core::Result<i32>;
    fn lParam3(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhoneDeviceSpecificEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>() -> ITPhoneDeviceSpecificEvent_Vtbl {
        unsafe extern "system" fn Phone<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).lParam1() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam1 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).lParam2() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).lParam3() {
                ::core::result::Result::Ok(ok__) => {
                    *pparam3 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Phone: Phone::<Identity, Impl, OFFSET>,
            lParam1: lParam1::<Identity, Impl, OFFSET>,
            lParam2: lParam2::<Identity, Impl, OFFSET>,
            lParam3: lParam3::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhoneDeviceSpecificEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPhoneEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Phone(&self) -> ::windows::core::Result<ITPhone>;
    fn Event(&self) -> ::windows::core::Result<PHONE_EVENT>;
    fn ButtonState(&self) -> ::windows::core::Result<PHONE_BUTTON_STATE>;
    fn HookSwitchState(&self) -> ::windows::core::Result<PHONE_HOOK_SWITCH_STATE>;
    fn HookSwitchDevice(&self) -> ::windows::core::Result<PHONE_HOOK_SWITCH_DEVICE>;
    fn RingMode(&self) -> ::windows::core::Result<i32>;
    fn ButtonLampId(&self) -> ::windows::core::Result<i32>;
    fn NumberGathered(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPhoneEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>() -> ITPhoneEvent_Vtbl {
        unsafe extern "system" fn Phone<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut PHONE_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonState<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ButtonState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HookSwitchState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchDevice<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HookSwitchDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *pdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingMode<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plringmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonLampId<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plbuttonlampid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ButtonLampId() {
                ::core::result::Result::Ok(ok__) => {
                    *plbuttonlampid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberGathered<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NumberGathered() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITPhoneEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Phone: Phone::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            ButtonState: ButtonState::<Identity, Impl, OFFSET>,
            HookSwitchState: HookSwitchState::<Identity, Impl, OFFSET>,
            HookSwitchDevice: HookSwitchDevice::<Identity, Impl, OFFSET>,
            RingMode: RingMode::<Identity, Impl, OFFSET>,
            ButtonLampId: ButtonLampId::<Identity, Impl, OFFSET>,
            NumberGathered: NumberGathered::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPhoneEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPluggableTerminalClassInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Company(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Version(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn TerminalClass(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CLSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Direction(&self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
    fn MediaTypes(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPluggableTerminalClassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>() -> ITPluggableTerminalClassInfo_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Company<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcompany: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Company() {
                ::core::result::Result::Ok(ok__) => {
                    *pcompany = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *pversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TerminalClass() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminalclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypes<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pmediatypes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Company: Company::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            TerminalClass: TerminalClass::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
            MediaTypes: MediaTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalClassInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalEventSink_Impl: Sized {
    fn FireEvent(&self, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalEventSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSink_Impl, const OFFSET: isize>() -> ITPluggableTerminalEventSink_Vtbl {
        unsafe extern "system" fn FireEvent<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FireEvent(::core::mem::transmute_copy(&pmspeventinfo)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), FireEvent: FireEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalEventSink as ::windows::core::Interface>::IID
    }
}
pub trait ITPluggableTerminalEventSinkRegistration_Impl: Sized {
    fn RegisterSink(&self, peventsink: &::core::option::Option<ITPluggableTerminalEventSink>) -> ::windows::core::Result<()>;
    fn UnregisterSink(&self) -> ::windows::core::Result<()>;
}
impl ITPluggableTerminalEventSinkRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: isize>() -> ITPluggableTerminalEventSinkRegistration_Vtbl {
        unsafe extern "system" fn RegisterSink<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterSink(::core::mem::transmute(&peventsink)).into()
        }
        unsafe extern "system" fn UnregisterSink<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterSink().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterSink: RegisterSink::<Identity, Impl, OFFSET>,
            UnregisterSink: UnregisterSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalEventSinkRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPluggableTerminalSuperclassInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CLSID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPluggableTerminalSuperclassInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: isize>() -> ITPluggableTerminalSuperclassInfo_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Identity: ::windows::core::IUnknownImpl, Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPluggableTerminalSuperclassInfo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITPrivateEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Address(&self) -> ::windows::core::Result<ITAddress>;
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn CallHub(&self) -> ::windows::core::Result<ITCallHub>;
    fn EventCode(&self) -> ::windows::core::Result<i32>;
    fn EventInterface(&self) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITPrivateEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITPrivateEvent_Impl, const OFFSET: isize>() -> ITPrivateEvent_Vtbl {
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Identity: ::windows::core::IUnknownImpl, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallHub() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventCode<Identity: ::windows::core::IUnknownImpl, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pleventcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventCode() {
                ::core::result::Result::Ok(ok__) => {
                    *pleventcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterface<Identity: ::windows::core::IUnknownImpl, Impl: ITPrivateEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *peventinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Address: Address::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            CallHub: CallHub::<Identity, Impl, OFFSET>,
            EventCode: EventCode::<Identity, Impl, OFFSET>,
            EventInterface: EventInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITPrivateEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQOSEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn Event(&self) -> ::windows::core::Result<QOS_EVENT>;
    fn MediaType(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQOSEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQOSEvent_Impl, const OFFSET: isize>() -> ITQOSEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITQOSEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITQOSEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pqosevent: *mut QOS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pqosevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Identity: ::windows::core::IUnknownImpl, Impl: ITQOSEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQOSEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQueue_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetMeasurementPeriod(&self, lperiod: i32) -> ::windows::core::Result<()>;
    fn MeasurementPeriod(&self) -> ::windows::core::Result<i32>;
    fn TotalCallsQueued(&self) -> ::windows::core::Result<i32>;
    fn CurrentCallsQueued(&self) -> ::windows::core::Result<i32>;
    fn TotalCallsAbandoned(&self) -> ::windows::core::Result<i32>;
    fn TotalCallsFlowedIn(&self) -> ::windows::core::Result<i32>;
    fn TotalCallsFlowedOut(&self) -> ::windows::core::Result<i32>;
    fn LongestEverWaitTime(&self) -> ::windows::core::Result<i32>;
    fn CurrentLongestWaitTime(&self) -> ::windows::core::Result<i32>;
    fn AverageWaitTime(&self) -> ::windows::core::Result<i32>;
    fn FinalDisposition(&self) -> ::windows::core::Result<i32>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>() -> ITQueue_Vtbl {
        unsafe extern "system" fn SetMeasurementPeriod<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMeasurementPeriod(::core::mem::transmute_copy(&lperiod)).into()
        }
        unsafe extern "system" fn MeasurementPeriod<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MeasurementPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *plperiod = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsQueued<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalCallsQueued() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCallsQueued<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentCallsQueued() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsAbandoned<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalCallsAbandoned() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedIn<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalCallsFlowedIn() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedOut<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TotalCallsFlowedOut() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestEverWaitTime<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LongestEverWaitTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLongestWaitTime<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CurrentLongestWaitTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWaitTime<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AverageWaitTime() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalDisposition<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FinalDisposition() {
                ::core::result::Result::Ok(ok__) => {
                    *plcalls = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMeasurementPeriod: SetMeasurementPeriod::<Identity, Impl, OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Identity, Impl, OFFSET>,
            TotalCallsQueued: TotalCallsQueued::<Identity, Impl, OFFSET>,
            CurrentCallsQueued: CurrentCallsQueued::<Identity, Impl, OFFSET>,
            TotalCallsAbandoned: TotalCallsAbandoned::<Identity, Impl, OFFSET>,
            TotalCallsFlowedIn: TotalCallsFlowedIn::<Identity, Impl, OFFSET>,
            TotalCallsFlowedOut: TotalCallsFlowedOut::<Identity, Impl, OFFSET>,
            LongestEverWaitTime: LongestEverWaitTime::<Identity, Impl, OFFSET>,
            CurrentLongestWaitTime: CurrentLongestWaitTime::<Identity, Impl, OFFSET>,
            AverageWaitTime: AverageWaitTime::<Identity, Impl, OFFSET>,
            FinalDisposition: FinalDisposition::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQueue as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITQueueEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Queue(&self) -> ::windows::core::Result<ITQueue>;
    fn Event(&self) -> ::windows::core::Result<ACDQUEUE_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITQueueEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITQueueEvent_Impl, const OFFSET: isize>() -> ITQueueEvent_Vtbl {
        unsafe extern "system" fn Queue<Identity: ::windows::core::IUnknownImpl, Impl: ITQueueEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Queue() {
                ::core::result::Result::Ok(ok__) => {
                    *ppqueue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITQueueEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDQUEUE_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Queue: Queue::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITQueueEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRendezvous_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DefaultDirectories(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDefaultDirectories(&self) -> ::windows::core::Result<IEnumDirectory>;
    fn CreateDirectory(&self, directorytype: DIRECTORY_TYPE, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITDirectory>;
    fn CreateDirectoryObject(&self, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &super::super::Foundation::BSTR) -> ::windows::core::Result<ITDirectoryObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRendezvous_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRendezvous_Impl, const OFFSET: isize>() -> ITRendezvous_Vtbl {
        unsafe extern "system" fn DefaultDirectories<Identity: ::windows::core::IUnknownImpl, Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DefaultDirectories() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDefaultDirectories<Identity: ::windows::core::IUnknownImpl, Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumdirectory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateDefaultDirectories() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumdirectory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectory<Identity: ::windows::core::IUnknownImpl, Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directorytype: DIRECTORY_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDirectory(::core::mem::transmute_copy(&directorytype), ::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdir = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectoryObject<Identity: ::windows::core::IUnknownImpl, Impl: ITRendezvous_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdirectoryobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDirectoryObject(::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdirectoryobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DefaultDirectories: DefaultDirectories::<Identity, Impl, OFFSET>,
            EnumerateDefaultDirectories: EnumerateDefaultDirectories::<Identity, Impl, OFFSET>,
            CreateDirectory: CreateDirectory::<Identity, Impl, OFFSET>,
            CreateDirectoryObject: CreateDirectoryObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRendezvous as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRequest_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MakeCall(&self, pdestaddress: &super::super::Foundation::BSTR, pappname: &super::super::Foundation::BSTR, pcalledparty: &super::super::Foundation::BSTR, pcomment: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRequest_Impl, const OFFSET: isize>() -> ITRequest_Vtbl {
        unsafe extern "system" fn MakeCall<Identity: ::windows::core::IUnknownImpl, Impl: ITRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalledparty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MakeCall(::core::mem::transmute(&pdestaddress), ::core::mem::transmute(&pappname), ::core::mem::transmute(&pcalledparty), ::core::mem::transmute(&pcomment)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), MakeCall: MakeCall::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRequest as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITRequestEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RegistrationInstance(&self) -> ::windows::core::Result<i32>;
    fn RequestMode(&self) -> ::windows::core::Result<i32>;
    fn DestAddress(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AppName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CalledParty(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Comment(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITRequestEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEvent_Impl, const OFFSET: isize>() -> ITRequestEvent_Vtbl {
        unsafe extern "system" fn RegistrationInstance<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plregistrationinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegistrationInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plregistrationinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMode<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plrequestmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestMode() {
                ::core::result::Result::Ok(ok__) => {
                    *plrequestmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestAddress<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DestAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdestaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppName<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppappname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalledParty<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcalledparty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CalledParty() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcalledparty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Identity: ::windows::core::IUnknownImpl, Impl: ITRequestEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegistrationInstance: RegistrationInstance::<Identity, Impl, OFFSET>,
            RequestMode: RequestMode::<Identity, Impl, OFFSET>,
            DestAddress: DestAddress::<Identity, Impl, OFFSET>,
            AppName: AppName::<Identity, Impl, OFFSET>,
            CalledParty: CalledParty::<Identity, Impl, OFFSET>,
            Comment: Comment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITRequestEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITScriptableAudioFormat_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Channels(&self) -> ::windows::core::Result<i32>;
    fn SetChannels(&self, nnewval: i32) -> ::windows::core::Result<()>;
    fn SamplesPerSec(&self) -> ::windows::core::Result<i32>;
    fn SetSamplesPerSec(&self, nnewval: i32) -> ::windows::core::Result<()>;
    fn AvgBytesPerSec(&self) -> ::windows::core::Result<i32>;
    fn SetAvgBytesPerSec(&self, nnewval: i32) -> ::windows::core::Result<()>;
    fn BlockAlign(&self) -> ::windows::core::Result<i32>;
    fn SetBlockAlign(&self, nnewval: i32) -> ::windows::core::Result<()>;
    fn BitsPerSample(&self) -> ::windows::core::Result<i32>;
    fn SetBitsPerSample(&self, nnewval: i32) -> ::windows::core::Result<()>;
    fn FormatTag(&self) -> ::windows::core::Result<i32>;
    fn SetFormatTag(&self, nnewval: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITScriptableAudioFormat_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>() -> ITScriptableAudioFormat_Vtbl {
        unsafe extern "system" fn Channels<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Channels() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChannels(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn SamplesPerSec<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SamplesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSamplesPerSec(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn AvgBytesPerSec<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AvgBytesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAvgBytesPerSec(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BlockAlign<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BlockAlign() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBlockAlign(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn BitsPerSample<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BitsPerSample() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBitsPerSample(::core::mem::transmute_copy(&nnewval)).into()
        }
        unsafe extern "system" fn FormatTag<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FormatTag() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Identity: ::windows::core::IUnknownImpl, Impl: ITScriptableAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFormatTag(::core::mem::transmute_copy(&nnewval)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Channels: Channels::<Identity, Impl, OFFSET>,
            SetChannels: SetChannels::<Identity, Impl, OFFSET>,
            SamplesPerSec: SamplesPerSec::<Identity, Impl, OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Identity, Impl, OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Identity, Impl, OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Identity, Impl, OFFSET>,
            BlockAlign: BlockAlign::<Identity, Impl, OFFSET>,
            SetBlockAlign: SetBlockAlign::<Identity, Impl, OFFSET>,
            BitsPerSample: BitsPerSample::<Identity, Impl, OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Identity, Impl, OFFSET>,
            FormatTag: FormatTag::<Identity, Impl, OFFSET>,
            SetFormatTag: SetFormatTag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITScriptableAudioFormat as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStaticAudioTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WaveId(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStaticAudioTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStaticAudioTerminal_Impl, const OFFSET: isize>() -> ITStaticAudioTerminal_Vtbl {
        unsafe extern "system" fn WaveId<Identity: ::windows::core::IUnknownImpl, Impl: ITStaticAudioTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plwaveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WaveId() {
                ::core::result::Result::Ok(ok__) => {
                    *plwaveid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), WaveId: WaveId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStaticAudioTerminal as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaType(&self) -> ::windows::core::Result<i32>;
    fn Direction(&self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StartStream(&self) -> ::windows::core::Result<()>;
    fn PauseStream(&self) -> ::windows::core::Result<()>;
    fn StopStream(&self) -> ::windows::core::Result<()>;
    fn SelectTerminal(&self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn UnselectTerminal(&self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn EnumerateTerminals(&self) -> ::windows::core::Result<IEnumTerminal>;
    fn Terminals(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>() -> ITStream_Vtbl {
        unsafe extern "system" fn MediaType<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptd: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *ptd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartStream<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartStream().into()
        }
        unsafe extern "system" fn PauseStream<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PauseStream().into()
        }
        unsafe extern "system" fn StopStream<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopStream().into()
        }
        unsafe extern "system" fn SelectTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SelectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnselectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Identity: ::windows::core::IUnknownImpl, Impl: ITStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Terminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            StartStream: StartStream::<Identity, Impl, OFFSET>,
            PauseStream: PauseStream::<Identity, Impl, OFFSET>,
            StopStream: StopStream::<Identity, Impl, OFFSET>,
            SelectTerminal: SelectTerminal::<Identity, Impl, OFFSET>,
            UnselectTerminal: UnselectTerminal::<Identity, Impl, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, Impl, OFFSET>,
            Terminals: Terminals::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITStreamControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateStream(&self, lmediatype: i32, td: TERMINAL_DIRECTION) -> ::windows::core::Result<ITStream>;
    fn RemoveStream(&self, pstream: &::core::option::Option<ITStream>) -> ::windows::core::Result<()>;
    fn EnumerateStreams(&self) -> ::windows::core::Result<IEnumStream>;
    fn Streams(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITStreamControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamControl_Impl, const OFFSET: isize>() -> ITStreamControl_Vtbl {
        unsafe extern "system" fn CreateStream<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateStream(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&td)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute(&pstream)).into()
        }
        unsafe extern "system" fn EnumerateStreams<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Streams<Identity: ::windows::core::IUnknownImpl, Impl: ITStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Streams() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            EnumerateStreams: EnumerateStreams::<Identity, Impl, OFFSET>,
            Streams: Streams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITStreamControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITSubStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartSubStream(&self) -> ::windows::core::Result<()>;
    fn PauseSubStream(&self) -> ::windows::core::Result<()>;
    fn StopSubStream(&self) -> ::windows::core::Result<()>;
    fn SelectTerminal(&self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn UnselectTerminal(&self, pterminal: &::core::option::Option<ITTerminal>) -> ::windows::core::Result<()>;
    fn EnumerateTerminals(&self) -> ::windows::core::Result<IEnumTerminal>;
    fn Terminals(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Stream(&self) -> ::windows::core::Result<ITStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITSubStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const OFFSET: isize>() -> ITSubStream_Vtbl {
        unsafe extern "system" fn StartSubStream<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartSubStream().into()
        }
        unsafe extern "system" fn PauseSubStream<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PauseSubStream().into()
        }
        unsafe extern "system" fn StopSubStream<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopSubStream().into()
        }
        unsafe extern "system" fn SelectTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SelectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn UnselectTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnselectTerminal(::core::mem::transmute(&pterminal)).into()
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Terminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminals = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppitstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppitstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartSubStream: StartSubStream::<Identity, Impl, OFFSET>,
            PauseSubStream: PauseSubStream::<Identity, Impl, OFFSET>,
            StopSubStream: StopSubStream::<Identity, Impl, OFFSET>,
            SelectTerminal: SelectTerminal::<Identity, Impl, OFFSET>,
            UnselectTerminal: UnselectTerminal::<Identity, Impl, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, Impl, OFFSET>,
            Terminals: Terminals::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSubStream as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITSubStreamControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateSubStream(&self) -> ::windows::core::Result<ITSubStream>;
    fn RemoveSubStream(&self, psubstream: &::core::option::Option<ITSubStream>) -> ::windows::core::Result<()>;
    fn EnumerateSubStreams(&self) -> ::windows::core::Result<IEnumSubStream>;
    fn SubStreams(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITSubStreamControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamControl_Impl, const OFFSET: isize>() -> ITSubStreamControl_Vtbl {
        unsafe extern "system" fn CreateSubStream<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSubStream() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubStream<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveSubStream(::core::mem::transmute(&psubstream)).into()
        }
        unsafe extern "system" fn EnumerateSubStreams<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateSubStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumsubstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubStreams<Identity: ::windows::core::IUnknownImpl, Impl: ITSubStreamControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateSubStream: CreateSubStream::<Identity, Impl, OFFSET>,
            RemoveSubStream: RemoveSubStream::<Identity, Impl, OFFSET>,
            EnumerateSubStreams: EnumerateSubStreams::<Identity, Impl, OFFSET>,
            SubStreams: SubStreams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITSubStreamControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPI_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self) -> ::windows::core::Result<()>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
    fn Addresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateAddresses(&self) -> ::windows::core::Result<IEnumAddress>;
    fn RegisterCallNotifications(&self, paddress: &::core::option::Option<ITAddress>, fmonitor: i16, fowner: i16, lmediatypes: i32, lcallbackinstance: i32) -> ::windows::core::Result<i32>;
    fn UnregisterNotifications(&self, lregister: i32) -> ::windows::core::Result<()>;
    fn CallHubs(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateCallHubs(&self) -> ::windows::core::Result<IEnumCallHub>;
    fn SetCallHubTracking(&self, paddresses: &super::super::System::Com::VARIANT, btracking: i16) -> ::windows::core::Result<()>;
    fn EnumeratePrivateTAPIObjects(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown>;
    fn PrivateTAPIObjects(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RegisterRequestRecipient(&self, lregistrationinstance: i32, lrequestmode: i32, fenable: i16) -> ::windows::core::Result<()>;
    fn SetAssistedTelephonyPriority(&self, pappfilename: &super::super::Foundation::BSTR, fpriority: i16) -> ::windows::core::Result<()>;
    fn SetApplicationPriority(&self, pappfilename: &super::super::Foundation::BSTR, lmediatype: i32, fpriority: i16) -> ::windows::core::Result<()>;
    fn SetEventFilter(&self, lfiltermask: i32) -> ::windows::core::Result<()>;
    fn EventFilter(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>() -> ITTAPI_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shutdown().into()
        }
        unsafe extern "system" fn Addresses<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Addresses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallNotifications<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, fmonitor: i16, fowner: i16, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterCallNotifications(::core::mem::transmute(&paddress), ::core::mem::transmute_copy(&fmonitor), ::core::mem::transmute_copy(&fowner), ::core::mem::transmute_copy(&lmediatypes), ::core::mem::transmute_copy(&lcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *plregister = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNotifications<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lregister: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterNotifications(::core::mem::transmute_copy(&lregister)).into()
        }
        unsafe extern "system" fn CallHubs<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallHubs() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallHubs<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateCallHubs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcallhub = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHubTracking<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, btracking: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCallHubTracking(::core::mem::transmute(&paddresses), ::core::mem::transmute_copy(&btracking)).into()
        }
        unsafe extern "system" fn EnumeratePrivateTAPIObjects<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumeratePrivateTAPIObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateTAPIObjects<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PrivateTAPIObjects() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRequestRecipient<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lregistrationinstance: i32, lrequestmode: i32, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterRequestRecipient(::core::mem::transmute_copy(&lregistrationinstance), ::core::mem::transmute_copy(&lrequestmode), ::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn SetAssistedTelephonyPriority<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAssistedTelephonyPriority(::core::mem::transmute(&pappfilename), ::core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetApplicationPriority<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationPriority(::core::mem::transmute(&pappfilename), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&fpriority)).into()
        }
        unsafe extern "system" fn SetEventFilter<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfiltermask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventFilter(::core::mem::transmute_copy(&lfiltermask)).into()
        }
        unsafe extern "system" fn EventFilter<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plfiltermask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *plfiltermask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            Addresses: Addresses::<Identity, Impl, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, Impl, OFFSET>,
            RegisterCallNotifications: RegisterCallNotifications::<Identity, Impl, OFFSET>,
            UnregisterNotifications: UnregisterNotifications::<Identity, Impl, OFFSET>,
            CallHubs: CallHubs::<Identity, Impl, OFFSET>,
            EnumerateCallHubs: EnumerateCallHubs::<Identity, Impl, OFFSET>,
            SetCallHubTracking: SetCallHubTracking::<Identity, Impl, OFFSET>,
            EnumeratePrivateTAPIObjects: EnumeratePrivateTAPIObjects::<Identity, Impl, OFFSET>,
            PrivateTAPIObjects: PrivateTAPIObjects::<Identity, Impl, OFFSET>,
            RegisterRequestRecipient: RegisterRequestRecipient::<Identity, Impl, OFFSET>,
            SetAssistedTelephonyPriority: SetAssistedTelephonyPriority::<Identity, Impl, OFFSET>,
            SetApplicationPriority: SetApplicationPriority::<Identity, Impl, OFFSET>,
            SetEventFilter: SetEventFilter::<Identity, Impl, OFFSET>,
            EventFilter: EventFilter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPI as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPI2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITTAPI_Impl {
    fn Phones(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePhones(&self) -> ::windows::core::Result<IEnumPhone>;
    fn CreateEmptyCollectionObject(&self) -> ::windows::core::Result<ITCollection2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPI2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI2_Impl, const OFFSET: isize>() -> ITTAPI2_Vtbl {
        unsafe extern "system" fn Phones<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Phones() {
                ::core::result::Result::Ok(ok__) => {
                    *pphones = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumeratePhones() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEmptyCollectionObject<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPI2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEmptyCollectionObject() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITTAPI_Vtbl::new::<Identity, Impl, OFFSET>(),
            Phones: Phones::<Identity, Impl, OFFSET>,
            EnumeratePhones: EnumeratePhones::<Identity, Impl, OFFSET>,
            CreateEmptyCollectionObject: CreateEmptyCollectionObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPI2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITTAPI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPICallCenter_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumerateAgentHandlers(&self) -> ::windows::core::Result<IEnumAgentHandler>;
    fn AgentHandlers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPICallCenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPICallCenter_Impl, const OFFSET: isize>() -> ITTAPICallCenter_Vtbl {
        unsafe extern "system" fn EnumerateAgentHandlers<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPICallCenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateAgentHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumhandler = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentHandlers<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPICallCenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AgentHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumerateAgentHandlers: EnumerateAgentHandlers::<Identity, Impl, OFFSET>,
            AgentHandlers: AgentHandlers::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIDispatchEventNotification_Impl, const OFFSET: isize>() -> ITTAPIDispatchEventNotification_Vtbl {
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIDispatchEventNotification as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIEventNotification_Impl: Sized {
    fn Event(&self, tapievent: TAPI_EVENT, pevent: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIEventNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIEventNotification_Impl, const OFFSET: isize>() -> ITTAPIEventNotification_Vtbl {
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIEventNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Event(::core::mem::transmute_copy(&tapievent), ::core::mem::transmute(&pevent)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Event: Event::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIEventNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIObjectEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn TAPIObject(&self) -> ::windows::core::Result<ITTAPI>;
    fn Event(&self) -> ::windows::core::Result<TAPIOBJECT_EVENT>;
    fn Address(&self) -> ::windows::core::Result<ITAddress>;
    fn CallbackInstance(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIObjectEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>() -> ITTAPIObjectEvent_Vtbl {
        unsafe extern "system" fn TAPIObject<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TAPIObject() {
                ::core::result::Result::Ok(ok__) => {
                    *pptapiobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut TAPIOBJECT_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Event() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            TAPIObject: TAPIObject::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTAPIObjectEvent2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITTAPIObjectEvent_Impl {
    fn Phone(&self) -> ::windows::core::Result<ITPhone>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTAPIObjectEvent2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent2_Impl, const OFFSET: isize>() -> ITTAPIObjectEvent2_Vtbl {
        unsafe extern "system" fn Phone<Identity: ::windows::core::IUnknownImpl, Impl: ITTAPIObjectEvent2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppphone = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ITTAPIObjectEvent_Vtbl::new::<Identity, Impl, OFFSET>(), Phone: Phone::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTAPIObjectEvent2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITTAPIObjectEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTTSTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&self) -> ::windows::core::Result<ITTerminal>;
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTTSTerminalEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>() -> ITTTSTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Identity: ::windows::core::IUnknownImpl, Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: ITTTSTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTTSTerminalEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminal_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn State(&self) -> ::windows::core::Result<TERMINAL_STATE>;
    fn TerminalType(&self) -> ::windows::core::Result<TERMINAL_TYPE>;
    fn TerminalClass(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn MediaType(&self) -> ::windows::core::Result<i32>;
    fn Direction(&self) -> ::windows::core::Result<TERMINAL_DIRECTION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminal_Impl, const OFFSET: isize>() -> ITTerminal_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalstate: *mut TERMINAL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *pterminalstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalType<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TERMINAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TerminalType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TerminalClass() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminalclass = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *plmediatype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *pdirection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            TerminalType: TerminalType::<Identity, Impl, OFFSET>,
            TerminalClass: TerminalClass::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminal as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalSupport_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StaticTerminals(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateStaticTerminals(&self) -> ::windows::core::Result<IEnumTerminal>;
    fn DynamicTerminalClasses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumerateDynamicTerminalClasses(&self) -> ::windows::core::Result<IEnumTerminalClass>;
    fn CreateTerminal(&self, pterminalclass: &super::super::Foundation::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
    fn GetDefaultStaticTerminal(&self, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport_Impl, const OFFSET: isize>() -> ITTerminalSupport_Vtbl {
        unsafe extern "system" fn StaticTerminals<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StaticTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateStaticTerminals<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateStaticTerminals() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminalenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicTerminalClasses<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DynamicTerminalClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDynamicTerminalClasses<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminalclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerateDynamicTerminalClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminalclassenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pterminalclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTerminal(::core::mem::transmute(&pterminalclass), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultStaticTerminal<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultStaticTerminal(::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StaticTerminals: StaticTerminals::<Identity, Impl, OFFSET>,
            EnumerateStaticTerminals: EnumerateStaticTerminals::<Identity, Impl, OFFSET>,
            DynamicTerminalClasses: DynamicTerminalClasses::<Identity, Impl, OFFSET>,
            EnumerateDynamicTerminalClasses: EnumerateDynamicTerminalClasses::<Identity, Impl, OFFSET>,
            CreateTerminal: CreateTerminal::<Identity, Impl, OFFSET>,
            GetDefaultStaticTerminal: GetDefaultStaticTerminal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminalSupport as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITTerminalSupport2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ITTerminalSupport_Impl {
    fn PluggableSuperclasses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePluggableSuperclasses(&self) -> ::windows::core::Result<IEnumPluggableSuperclassInfo>;
    fn PluggableTerminalClasses(&self, bstrterminalsuperclass: &super::super::Foundation::BSTR, lmediatype: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumeratePluggableTerminalClasses(&self, iidterminalsuperclass: &::windows::core::GUID, lmediatype: i32) -> ::windows::core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITTerminalSupport2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>() -> ITTerminalSupport2_Vtbl {
        unsafe extern "system" fn PluggableSuperclasses<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PluggableSuperclasses() {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableSuperclasses<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsuperclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumeratePluggableSuperclasses() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsuperclassenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PluggableTerminalClasses<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrterminalsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PluggableTerminalClasses(::core::mem::transmute(&bstrterminalsuperclass), ::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableTerminalClasses<Identity: ::windows::core::IUnknownImpl, Impl: ITTerminalSupport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iidterminalsuperclass: ::windows::core::GUID, lmediatype: i32, ppclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumeratePluggableTerminalClasses(::core::mem::transmute(&iidterminalsuperclass), ::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclassenumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ITTerminalSupport_Vtbl::new::<Identity, Impl, OFFSET>(),
            PluggableSuperclasses: PluggableSuperclasses::<Identity, Impl, OFFSET>,
            EnumeratePluggableSuperclasses: EnumeratePluggableSuperclasses::<Identity, Impl, OFFSET>,
            PluggableTerminalClasses: PluggableTerminalClasses::<Identity, Impl, OFFSET>,
            EnumeratePluggableTerminalClasses: EnumeratePluggableTerminalClasses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITTerminalSupport2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ITTerminalSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITToneDetectionEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn AppSpecific(&self) -> ::windows::core::Result<i32>;
    fn TickCount(&self) -> ::windows::core::Result<i32>;
    fn CallbackInstance(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITToneDetectionEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>() -> ITToneDetectionEvent_Vtbl {
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcallinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows::core::IUnknownImpl, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppSpecific() {
                ::core::result::Result::Ok(ok__) => {
                    *plappspecific = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: ::windows::core::IUnknownImpl, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pltickcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows::core::IUnknownImpl, Impl: ITToneDetectionEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CallbackInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *plcallbackinstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Call: Call::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITToneDetectionEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITToneTerminalEvent_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Terminal(&self) -> ::windows::core::Result<ITTerminal>;
    fn Call(&self) -> ::windows::core::Result<ITCallInfo>;
    fn Error(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITToneTerminalEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>() -> ITToneTerminalEvent_Vtbl {
        unsafe extern "system" fn Terminal<Identity: ::windows::core::IUnknownImpl, Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Terminal() {
                ::core::result::Result::Ok(ok__) => {
                    *ppterminal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Identity: ::windows::core::IUnknownImpl, Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Call() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcall = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: ITToneTerminalEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *phrerrorcode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITToneTerminalEvent as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub trait ITnef_Impl: Sized {
    fn AddProps(&self, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::Result<()>;
    fn ExtractProps(&self, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()>;
    fn Finish(&self, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()>;
    fn OpenTaggedBody(&self, lpmessage: &::core::option::Option<super::super::System::AddressBook::IMessage>, ulflags: u32) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetProps(&self, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::Result<()>;
    fn EncodeRecips(&self, ulflags: u32, lprecipienttable: &::core::option::Option<super::super::System::AddressBook::IMAPITable>) -> ::windows::core::Result<()>;
    fn FinishComponent(&self, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
impl ITnef_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITnef_Impl, const OFFSET: isize>() -> ITnef_Vtbl {
        unsafe extern "system" fn AddProps<Identity: ::windows::core::IUnknownImpl, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddProps(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulelemid), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&lpproplist)).into()
        }
        unsafe extern "system" fn ExtractProps<Identity: ::windows::core::IUnknownImpl, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ExtractProps(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpproplist), ::core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn Finish<Identity: ::windows::core::IUnknownImpl, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Finish(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpkey), ::core::mem::transmute_copy(&lpproblems)).into()
        }
        unsafe extern "system" fn OpenTaggedBody<Identity: ::windows::core::IUnknownImpl, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpmessage: ::windows::core::RawPtr, ulflags: u32, lppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenTaggedBody(::core::mem::transmute(&lpmessage), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *lppstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProps<Identity: ::windows::core::IUnknownImpl, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProps(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulelemid), ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpprops)).into()
        }
        unsafe extern "system" fn EncodeRecips<Identity: ::windows::core::IUnknownImpl, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, lprecipienttable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EncodeRecips(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute(&lprecipienttable)).into()
        }
        unsafe extern "system" fn FinishComponent<Identity: ::windows::core::IUnknownImpl, Impl: ITnef_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FinishComponent(::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulcomponentid), ::core::mem::transmute_copy(&lpcustomproplist), ::core::mem::transmute_copy(&lpcustomprops), ::core::mem::transmute_copy(&lpproplist), ::core::mem::transmute_copy(&lpproblems)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddProps: AddProps::<Identity, Impl, OFFSET>,
            ExtractProps: ExtractProps::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            OpenTaggedBody: OpenTaggedBody::<Identity, Impl, OFFSET>,
            SetProps: SetProps::<Identity, Impl, OFFSET>,
            EncodeRecips: EncodeRecips::<Identity, Impl, OFFSET>,
            FinishComponent: FinishComponent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITnef as ::windows::core::Interface>::IID
    }
}
