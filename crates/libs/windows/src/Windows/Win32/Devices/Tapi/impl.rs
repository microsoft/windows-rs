pub trait IEnumACDGroupImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumACDGroup {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumACDGroup";
}
impl IEnumACDGroupVtbl {
    pub const fn new<Impl: IEnumACDGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumACDGroupVtbl {
        unsafe extern "system" fn Next<Impl: IEnumACDGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumACDGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumACDGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumACDGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumACDGroup>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumAddressImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumAddress {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumAddress";
}
impl IEnumAddressVtbl {
    pub const fn new<Impl: IEnumAddressImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumAddressVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumAddress>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumAgentImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumAgent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumAgent";
}
impl IEnumAgentVtbl {
    pub const fn new<Impl: IEnumAgentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumAgentVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumAgent>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumAgentHandlerImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumAgentHandler {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumAgentHandler";
}
impl IEnumAgentHandlerVtbl {
    pub const fn new<Impl: IEnumAgentHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumAgentHandlerVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumAgentHandler>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumAgentSessionImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumAgentSession {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumAgentSession";
}
impl IEnumAgentSessionVtbl {
    pub const fn new<Impl: IEnumAgentSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumAgentSessionVtbl {
        unsafe extern "system" fn Next<Impl: IEnumAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumAgentSession>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumBstrImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumBstr {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumBstr";
}
impl IEnumBstrVtbl {
    pub const fn new<Impl: IEnumBstrImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumBstrVtbl {
        unsafe extern "system" fn Next<Impl: IEnumBstrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppstrings: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppstrings), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumBstrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumBstrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumBstrImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumBstr>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumCallImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumCall {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumCall";
}
impl IEnumCallVtbl {
    pub const fn new<Impl: IEnumCallImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumCallVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumCall>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumCallHubImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumCallHub {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumCallHub";
}
impl IEnumCallHubVtbl {
    pub const fn new<Impl: IEnumCallHubImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumCallHubVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallHubImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallHubImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallHubImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallHubImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumCallHub>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumCallingCardImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumCallingCard {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumCallingCard";
}
impl IEnumCallingCardVtbl {
    pub const fn new<Impl: IEnumCallingCardImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumCallingCardVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumCallingCard>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumDialableAddrsImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDialableAddrs {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumDialableAddrs";
}
impl IEnumDialableAddrsVtbl {
    pub const fn new<Impl: IEnumDialableAddrsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumDialableAddrsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDialableAddrsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut super::super::Foundation::BSTR, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDialableAddrsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDialableAddrsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumDialableAddrsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumDialableAddrs>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumDirectoryImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDirectory {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumDirectory";
}
impl IEnumDirectoryVtbl {
    pub const fn new<Impl: IEnumDirectoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumDirectoryVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumDirectory>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumDirectoryObjectImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDirectoryObject {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumDirectoryObject";
}
impl IEnumDirectoryObjectVtbl {
    pub const fn new<Impl: IEnumDirectoryObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumDirectoryObjectVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pval: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&pval), pcfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumDirectoryObject>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumLocationImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumLocation {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumLocation";
}
impl IEnumLocationVtbl {
    pub const fn new<Impl: IEnumLocationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumLocationVtbl {
        unsafe extern "system" fn Next<Impl: IEnumLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumLocation>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumMcastScopeImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumMcastScope {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumMcastScope";
}
impl IEnumMcastScopeVtbl {
    pub const fn new<Impl: IEnumMcastScopeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumMcastScopeVtbl {
        unsafe extern "system" fn Next<Impl: IEnumMcastScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppscopes: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppscopes), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumMcastScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumMcastScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumMcastScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumMcastScope>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumPhoneImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumPhone {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumPhone";
}
impl IEnumPhoneVtbl {
    pub const fn new<Impl: IEnumPhoneImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumPhoneVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumPhone>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumPluggableSuperclassInfoImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumPluggableSuperclassInfo {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumPluggableSuperclassInfo";
}
impl IEnumPluggableSuperclassInfoVtbl {
    pub const fn new<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumPluggableSuperclassInfoVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumPluggableSuperclassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumPluggableSuperclassInfo>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumPluggableTerminalClassInfoImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumPluggableTerminalClassInfo {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumPluggableTerminalClassInfo";
}
impl IEnumPluggableTerminalClassInfoVtbl {
    pub const fn new<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumPluggableTerminalClassInfoVtbl {
        unsafe extern "system" fn Next<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumPluggableTerminalClassInfo>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumQueueImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumQueue {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumQueue";
}
impl IEnumQueueVtbl {
    pub const fn new<Impl: IEnumQueueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumQueueVtbl {
        unsafe extern "system" fn Next<Impl: IEnumQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumQueue>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumStreamImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumStream {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumStream";
}
impl IEnumStreamVtbl {
    pub const fn new<Impl: IEnumStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumStreamVtbl {
        unsafe extern "system" fn Next<Impl: IEnumStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumStream>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumSubStreamImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumSubStream {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumSubStream";
}
impl IEnumSubStreamVtbl {
    pub const fn new<Impl: IEnumSubStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumSubStreamVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumSubStream>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumTerminalImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumTerminal {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumTerminal";
}
impl IEnumTerminalVtbl {
    pub const fn new<Impl: IEnumTerminalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumTerminalVtbl {
        unsafe extern "system" fn Next<Impl: IEnumTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&ppelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumTerminal>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumTerminalClassImpl: Sized {
    fn Next();
    fn Reset();
    fn Skip();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumTerminalClass {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IEnumTerminalClass";
}
impl IEnumTerminalClassVtbl {
    pub const fn new<Impl: IEnumTerminalClassImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnumTerminalClassVtbl {
        unsafe extern "system" fn Next<Impl: IEnumTerminalClassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pelements: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&pelements), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumTerminalClassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumTerminalClassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumTerminalClassImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnumTerminalClass>, base.5, Next::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMcastAddressAllocationImpl: Sized + IDispatchImpl {
    fn Scopes();
    fn EnumerateScopes();
    fn RequestAddress();
    fn RenewAddress();
    fn ReleaseAddress();
    fn CreateLeaseInfo();
    fn CreateLeaseInfoFromVariant();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMcastAddressAllocation {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IMcastAddressAllocation";
}
#[cfg(feature = "Win32_System_Com")]
impl IMcastAddressAllocationVtbl {
    pub const fn new<Impl: IMcastAddressAllocationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMcastAddressAllocationVtbl {
        unsafe extern "system" fn Scopes<Impl: IMcastAddressAllocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scopes(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateScopes<Impl: IMcastAddressAllocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenummcastscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateScopes(::core::mem::transmute_copy(&ppenummcastscope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAddress<Impl: IMcastAddressAllocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pscope: ::windows::core::RawPtr, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAddress(&*(&pscope as *const <IMcastScope as ::windows::core::Abi>::Abi as *const <IMcastScope as ::windows::core::DefaultType>::DefaultType), leasestarttime, leasestoptime, numaddresses, ::core::mem::transmute_copy(&ppleaseresponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewAddress<Impl: IMcastAddressAllocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lreserved: i32, prenewrequest: ::windows::core::RawPtr, pprenewresponse: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenewAddress(lreserved, &*(&prenewrequest as *const <IMcastLeaseInfo as ::windows::core::Abi>::Abi as *const <IMcastLeaseInfo as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprenewresponse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseAddress<Impl: IMcastAddressAllocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preleaserequest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseAddress(&*(&preleaserequest as *const <IMcastLeaseInfo as ::windows::core::Abi>::Abi as *const <IMcastLeaseInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLeaseInfo<Impl: IMcastAddressAllocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const super::super::Foundation::PWSTR, prequestid: super::super::Foundation::PWSTR, pserveraddress: super::super::Foundation::PWSTR, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLeaseInfo(
                leasestarttime,
                leasestoptime,
                dwnumaddresses,
                &*(&ppaddresses as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&prequestid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pserveraddress as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppreleaserequest),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLeaseInfoFromVariant<Impl: IMcastAddressAllocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, vaddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, prequestid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pserveraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppreleaserequest: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLeaseInfoFromVariant(
                leasestarttime,
                leasestoptime,
                &*(&vaddresses as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&prequestid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pserveraddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppreleaserequest),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMcastAddressAllocation>, base.5, Scopes::<Impl, OFFSET>, EnumerateScopes::<Impl, OFFSET>, RequestAddress::<Impl, OFFSET>, RenewAddress::<Impl, OFFSET>, ReleaseAddress::<Impl, OFFSET>, CreateLeaseInfo::<Impl, OFFSET>, CreateLeaseInfoFromVariant::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMcastLeaseInfoImpl: Sized + IDispatchImpl {
    fn RequestID();
    fn LeaseStartTime();
    fn SetLeaseStartTime();
    fn LeaseStopTime();
    fn SetLeaseStopTime();
    fn AddressCount();
    fn ServerAddress();
    fn TTL();
    fn Addresses();
    fn EnumerateAddresses();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMcastLeaseInfo {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IMcastLeaseInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl IMcastLeaseInfoVtbl {
    pub const fn new<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMcastLeaseInfoVtbl {
        unsafe extern "system" fn RequestID<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprequestid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestID(::core::mem::transmute_copy(&pprequestid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseStartTime<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LeaseStartTime(::core::mem::transmute_copy(&ptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStartTime<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLeaseStartTime(time) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LeaseStopTime<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LeaseStopTime(::core::mem::transmute_copy(&ptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLeaseStopTime<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLeaseStopTime(time) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressCount<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddressCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerAddress<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerAddress(::core::mem::transmute_copy(&ppaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TTL(::core::mem::transmute_copy(&pttl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Addresses<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Addresses(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: IMcastLeaseInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddresses: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateAddresses(::core::mem::transmute_copy(&ppenumaddresses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMcastLeaseInfo>, base.5, RequestID::<Impl, OFFSET>, LeaseStartTime::<Impl, OFFSET>, SetLeaseStartTime::<Impl, OFFSET>, LeaseStopTime::<Impl, OFFSET>, SetLeaseStopTime::<Impl, OFFSET>, AddressCount::<Impl, OFFSET>, ServerAddress::<Impl, OFFSET>, TTL::<Impl, OFFSET>, Addresses::<Impl, OFFSET>, EnumerateAddresses::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMcastScopeImpl: Sized + IDispatchImpl {
    fn ScopeID();
    fn ServerID();
    fn InterfaceID();
    fn ScopeDescription();
    fn TTL();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMcastScope {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.IMcastScope";
}
#[cfg(feature = "Win32_System_Com")]
impl IMcastScopeVtbl {
    pub const fn new<Impl: IMcastScopeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMcastScopeVtbl {
        unsafe extern "system" fn ScopeID<Impl: IMcastScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScopeID(::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerID<Impl: IMcastScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerID(::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Impl: IMcastScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InterfaceID(::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeDescription<Impl: IMcastScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScopeDescription(::core::mem::transmute_copy(&ppdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TTL<Impl: IMcastScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TTL(::core::mem::transmute_copy(&pttl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMcastScope>, base.5, ScopeID::<Impl, OFFSET>, ServerID::<Impl, OFFSET>, InterfaceID::<Impl, OFFSET>, ScopeDescription::<Impl, OFFSET>, TTL::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITACDGroupImpl: Sized + IDispatchImpl {
    fn Name();
    fn EnumerateQueues();
    fn Queues();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITACDGroup {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITACDGroup";
}
#[cfg(feature = "Win32_System_Com")]
impl ITACDGroupVtbl {
    pub const fn new<Impl: ITACDGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITACDGroupVtbl {
        unsafe extern "system" fn Name<Impl: ITACDGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateQueues<Impl: ITACDGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateQueues(::core::mem::transmute_copy(&ppenumqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Queues<Impl: ITACDGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Queues(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITACDGroup>, base.5, Name::<Impl, OFFSET>, EnumerateQueues::<Impl, OFFSET>, Queues::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITACDGroupEventImpl: Sized + IDispatchImpl {
    fn Group();
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITACDGroupEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITACDGroupEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITACDGroupEventVtbl {
    pub const fn new<Impl: ITACDGroupEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITACDGroupEventVtbl {
        unsafe extern "system" fn Group<Impl: ITACDGroupEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Group(::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITACDGroupEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDGROUP_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITACDGroupEvent>, base.5, Group::<Impl, OFFSET>, Event::<Impl, OFFSET>)
    }
}
pub trait ITAMMediaFormatImpl: Sized {
    fn MediaFormat();
    fn SetMediaFormat();
}
impl ::windows::core::RuntimeName for ITAMMediaFormat {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAMMediaFormat";
}
impl ITAMMediaFormatVtbl {
    pub const fn new<Impl: ITAMMediaFormatImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAMMediaFormatVtbl {
        unsafe extern "system" fn MediaFormat<Impl: ITAMMediaFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaFormat(::core::mem::transmute_copy(&ppmt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaFormat<Impl: ITAMMediaFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMediaFormat(&*(&pmt as *const <super::super::Media::DirectShow::AM_MEDIA_TYPE as ::windows::core::Abi>::Abi as *const <super::super::Media::DirectShow::AM_MEDIA_TYPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAMMediaFormat>, base.5, MediaFormat::<Impl, OFFSET>, SetMediaFormat::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITASRTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Call();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITASRTerminalEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITASRTerminalEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITASRTerminalEventVtbl {
    pub const fn new<Impl: ITASRTerminalEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITASRTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITASRTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminal(::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITASRTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITASRTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error(::core::mem::transmute_copy(&phrerrorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITASRTerminalEvent>, base.5, Terminal::<Impl, OFFSET>, Call::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressImpl: Sized + IDispatchImpl {
    fn State();
    fn AddressName();
    fn ServiceProviderName();
    fn TAPIObject();
    fn CreateCall();
    fn Calls();
    fn EnumerateCalls();
    fn DialableAddress();
    fn CreateForwardInfoObject();
    fn Forward();
    fn CurrentForwardInfo();
    fn SetMessageWaiting();
    fn MessageWaiting();
    fn SetDoNotDisturb();
    fn DoNotDisturb();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAddress {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAddress";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressVtbl {
    pub const fn new<Impl: ITAddressImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAddressVtbl {
        unsafe extern "system" fn State<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddressstate: *mut ADDRESS_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&paddressstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressName<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddressName(::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProviderName<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceProviderName(::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TAPIObject<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TAPIObject(::core::mem::transmute_copy(&pptapiobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCall<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, laddresstype: i32, lmediatypes: i32, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCall(&*(&pdestaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), laddresstype, lmediatypes, ::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Calls(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCalls<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateCalls(::core::mem::transmute_copy(&ppcallenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialableAddress<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdialableaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DialableAddress(::core::mem::transmute_copy(&pdialableaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForwardInfoObject<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForwardInfoObject(::core::mem::transmute_copy(&ppforwardinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forward<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pforwardinfo: ::windows::core::RawPtr, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Forward(&*(&pforwardinfo as *const <ITForwardInformation as ::windows::core::Abi>::Abi as *const <ITForwardInformation as ::windows::core::DefaultType>::DefaultType), &*(&pcall as *const <ITBasicCallControl as ::windows::core::Abi>::Abi as *const <ITBasicCallControl as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentForwardInfo<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentForwardInfo(::core::mem::transmute_copy(&ppforwardinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageWaiting<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmessagewaiting: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMessageWaiting(fmessagewaiting) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageWaiting<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfmessagewaiting: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MessageWaiting(::core::mem::transmute_copy(&pfmessagewaiting)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotDisturb<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdonotdisturb: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDoNotDisturb(fdonotdisturb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotDisturb<Impl: ITAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdonotdisturb: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DoNotDisturb(::core::mem::transmute_copy(&pfdonotdisturb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITAddress>,
            base.5,
            State::<Impl, OFFSET>,
            AddressName::<Impl, OFFSET>,
            ServiceProviderName::<Impl, OFFSET>,
            TAPIObject::<Impl, OFFSET>,
            CreateCall::<Impl, OFFSET>,
            Calls::<Impl, OFFSET>,
            EnumerateCalls::<Impl, OFFSET>,
            DialableAddress::<Impl, OFFSET>,
            CreateForwardInfoObject::<Impl, OFFSET>,
            Forward::<Impl, OFFSET>,
            CurrentForwardInfo::<Impl, OFFSET>,
            SetMessageWaiting::<Impl, OFFSET>,
            MessageWaiting::<Impl, OFFSET>,
            SetDoNotDisturb::<Impl, OFFSET>,
            DoNotDisturb::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddress2Impl: Sized + ITAddressImpl + IDispatchImpl {
    fn Phones();
    fn EnumeratePhones();
    fn GetPhoneFromTerminal();
    fn PreferredPhones();
    fn EnumeratePreferredPhones();
    fn EventFilter();
    fn SetEventFilter();
    fn DeviceSpecific();
    fn DeviceSpecificVariant();
    fn NegotiateExtVersion();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAddress2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAddress2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAddress2Vtbl {
    pub const fn new<Impl: ITAddress2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAddress2Vtbl {
        unsafe extern "system" fn Phones<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Phones(::core::mem::transmute_copy(&pphones)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumeratePhones(::core::mem::transmute_copy(&ppenumphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneFromTerminal<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPhoneFromTerminal(&*(&pterminal as *const <ITTerminal as ::windows::core::Abi>::Abi as *const <ITTerminal as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredPhones<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredPhones(::core::mem::transmute_copy(&pphones)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredPhones<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumeratePreferredPhones(::core::mem::transmute_copy(&ppenumphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventFilter<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EventFilter(tapievent, lsubevent, ::core::mem::transmute_copy(&penable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEventFilter(tapievent, lsubevent, benable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSpecific<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceSpecific(&*(&pcall as *const <ITCallInfo as ::windows::core::Abi>::Abi as *const <ITCallInfo as ::windows::core::DefaultType>::DefaultType), pparams, dwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSpecificVariant<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceSpecificVariant(&*(&pcall as *const <ITCallInfo as ::windows::core::Abi>::Abi as *const <ITCallInfo as ::windows::core::DefaultType>::DefaultType), &*(&vardevspecificbytearray as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegotiateExtVersion<Impl: ITAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegotiateExtVersion(llowversion, lhighversion, ::core::mem::transmute_copy(&plextversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAddress2>, base.5, Phones::<Impl, OFFSET>, EnumeratePhones::<Impl, OFFSET>, GetPhoneFromTerminal::<Impl, OFFSET>, PreferredPhones::<Impl, OFFSET>, EnumeratePreferredPhones::<Impl, OFFSET>, EventFilter::<Impl, OFFSET>, SetEventFilter::<Impl, OFFSET>, DeviceSpecific::<Impl, OFFSET>, DeviceSpecificVariant::<Impl, OFFSET>, NegotiateExtVersion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressCapabilitiesImpl: Sized + IDispatchImpl {
    fn AddressCapability();
    fn AddressCapabilityString();
    fn CallTreatments();
    fn EnumerateCallTreatments();
    fn CompletionMessages();
    fn EnumerateCompletionMessages();
    fn DeviceClasses();
    fn EnumerateDeviceClasses();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAddressCapabilities {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAddressCapabilities";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressCapabilitiesVtbl {
    pub const fn new<Impl: ITAddressCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAddressCapabilitiesVtbl {
        unsafe extern "system" fn AddressCapability<Impl: ITAddressCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddressCapability(addresscap, ::core::mem::transmute_copy(&plcapability)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressCapabilityString<Impl: ITAddressCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddressCapabilityString(addresscapstring, ::core::mem::transmute_copy(&ppcapabilitystring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallTreatments<Impl: ITAddressCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallTreatments(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallTreatments<Impl: ITAddressCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcalltreatment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateCallTreatments(::core::mem::transmute_copy(&ppenumcalltreatment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompletionMessages<Impl: ITAddressCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompletionMessages(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCompletionMessages<Impl: ITAddressCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcompletionmessage: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateCompletionMessages(::core::mem::transmute_copy(&ppenumcompletionmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceClasses<Impl: ITAddressCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceClasses(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDeviceClasses<Impl: ITAddressCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdeviceclass: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateDeviceClasses(::core::mem::transmute_copy(&ppenumdeviceclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAddressCapabilities>, base.5, AddressCapability::<Impl, OFFSET>, AddressCapabilityString::<Impl, OFFSET>, CallTreatments::<Impl, OFFSET>, EnumerateCallTreatments::<Impl, OFFSET>, CompletionMessages::<Impl, OFFSET>, EnumerateCompletionMessages::<Impl, OFFSET>, DeviceClasses::<Impl, OFFSET>, EnumerateDeviceClasses::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressDeviceSpecificEventImpl: Sized + IDispatchImpl {
    fn Address();
    fn Call();
    fn lParam1();
    fn lParam2();
    fn lParam3();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAddressDeviceSpecificEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAddressDeviceSpecificEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressDeviceSpecificEventVtbl {
    pub const fn new<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAddressDeviceSpecificEventVtbl {
        unsafe extern "system" fn Address<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address(::core::mem::transmute_copy(&ppaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).lParam1(::core::mem::transmute_copy(&pparam1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).lParam2(::core::mem::transmute_copy(&pparam2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Impl: ITAddressDeviceSpecificEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).lParam3(::core::mem::transmute_copy(&pparam3)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAddressDeviceSpecificEvent>, base.5, Address::<Impl, OFFSET>, Call::<Impl, OFFSET>, lParam1::<Impl, OFFSET>, lParam2::<Impl, OFFSET>, lParam3::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressEventImpl: Sized + IDispatchImpl {
    fn Address();
    fn Event();
    fn Terminal();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAddressEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAddressEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressEventVtbl {
    pub const fn new<Impl: ITAddressEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAddressEventVtbl {
        unsafe extern "system" fn Address<Impl: ITAddressEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address(::core::mem::transmute_copy(&ppaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAddressEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut ADDRESS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Impl: ITAddressEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminal(::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAddressEvent>, base.5, Address::<Impl, OFFSET>, Event::<Impl, OFFSET>, Terminal::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressTranslationImpl: Sized + IDispatchImpl {
    fn TranslateAddress();
    fn TranslateDialog();
    fn EnumerateLocations();
    fn Locations();
    fn EnumerateCallingCards();
    fn CallingCards();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAddressTranslation {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAddressTranslation";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressTranslationVtbl {
    pub const fn new<Impl: ITAddressTranslationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAddressTranslationVtbl {
        unsafe extern "system" fn TranslateAddress<Impl: ITAddressTranslationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddresstotranslate: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lcard: i32, ltranslateoptions: i32, pptranslated: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslateAddress(&*(&paddresstotranslate as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lcard, ltranslateoptions, ::core::mem::transmute_copy(&pptranslated)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslateDialog<Impl: ITAddressTranslationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndowner: isize, paddressin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslateDialog(hwndowner, &*(&paddressin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateLocations<Impl: ITAddressTranslationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumlocation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateLocations(::core::mem::transmute_copy(&ppenumlocation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Locations<Impl: ITAddressTranslationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Locations(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallingCards<Impl: ITAddressTranslationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcallingcard: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateCallingCards(::core::mem::transmute_copy(&ppenumcallingcard)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallingCards<Impl: ITAddressTranslationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallingCards(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAddressTranslation>, base.5, TranslateAddress::<Impl, OFFSET>, TranslateDialog::<Impl, OFFSET>, EnumerateLocations::<Impl, OFFSET>, Locations::<Impl, OFFSET>, EnumerateCallingCards::<Impl, OFFSET>, CallingCards::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAddressTranslationInfoImpl: Sized + IDispatchImpl {
    fn DialableString();
    fn DisplayableString();
    fn CurrentCountryCode();
    fn DestinationCountryCode();
    fn TranslationResults();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAddressTranslationInfo {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAddressTranslationInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAddressTranslationInfoVtbl {
    pub const fn new<Impl: ITAddressTranslationInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAddressTranslationInfoVtbl {
        unsafe extern "system" fn DialableString<Impl: ITAddressTranslationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdialablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DialableString(::core::mem::transmute_copy(&ppdialablestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayableString<Impl: ITAddressTranslationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdisplayablestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayableString(::core::mem::transmute_copy(&ppdisplayablestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCountryCode<Impl: ITAddressTranslationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentCountryCode(::core::mem::transmute_copy(&countrycode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestinationCountryCode<Impl: ITAddressTranslationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DestinationCountryCode(::core::mem::transmute_copy(&countrycode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TranslationResults<Impl: ITAddressTranslationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plresults: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslationResults(::core::mem::transmute_copy(&plresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAddressTranslationInfo>, base.5, DialableString::<Impl, OFFSET>, DisplayableString::<Impl, OFFSET>, CurrentCountryCode::<Impl, OFFSET>, DestinationCountryCode::<Impl, OFFSET>, TranslationResults::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentImpl: Sized + IDispatchImpl {
    fn EnumerateAgentSessions();
    fn CreateSession();
    fn CreateSessionWithPIN();
    fn ID();
    fn User();
    fn SetState();
    fn State();
    fn SetMeasurementPeriod();
    fn MeasurementPeriod();
    fn OverallCallRate();
    fn NumberOfACDCalls();
    fn NumberOfIncomingCalls();
    fn NumberOfOutgoingCalls();
    fn TotalACDTalkTime();
    fn TotalACDCallTime();
    fn TotalWrapUpTime();
    fn AgentSessions();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAgent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAgent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentVtbl {
    pub const fn new<Impl: ITAgentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAgentVtbl {
        unsafe extern "system" fn EnumerateAgentSessions<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateAgentSessions(::core::mem::transmute_copy(&ppenumagentsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSession<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSession(&*(&pacdgroup as *const <ITACDGroup as ::windows::core::Abi>::Abi as *const <ITACDGroup as ::windows::core::DefaultType>::DefaultType), &*(&paddress as *const <ITAddress as ::windows::core::Abi>::Abi as *const <ITAddress as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppagentsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionWithPIN<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pacdgroup: ::windows::core::RawPtr, paddress: ::windows::core::RawPtr, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagentsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSessionWithPIN(
                &*(&pacdgroup as *const <ITACDGroup as ::windows::core::Abi>::Abi as *const <ITACDGroup as ::windows::core::DefaultType>::DefaultType),
                &*(&paddress as *const <ITAddress as ::windows::core::Abi>::Abi as *const <ITAddress as ::windows::core::DefaultType>::DefaultType),
                &*(&ppin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppagentsession),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ID<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ID(::core::mem::transmute_copy(&ppid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppuser: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User(::core::mem::transmute_copy(&ppuser)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, agentstate: AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetState(agentstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagentstate: *mut AGENT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pagentstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMeasurementPeriod<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMeasurementPeriod(lperiod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasurementPeriod<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MeasurementPeriod(::core::mem::transmute_copy(&plperiod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverallCallRate<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverallCallRate(::core::mem::transmute_copy(&pcycallrate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfACDCalls<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberOfACDCalls(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfIncomingCalls<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberOfIncomingCalls(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfOutgoingCalls<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberOfOutgoingCalls(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDTalkTime<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalACDTalkTime(::core::mem::transmute_copy(&pltalktime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalACDCallTime<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalACDCallTime(::core::mem::transmute_copy(&plcalltime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalWrapUpTime(::core::mem::transmute_copy(&plwrapuptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentSessions<Impl: ITAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AgentSessions(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITAgent>,
            base.5,
            EnumerateAgentSessions::<Impl, OFFSET>,
            CreateSession::<Impl, OFFSET>,
            CreateSessionWithPIN::<Impl, OFFSET>,
            ID::<Impl, OFFSET>,
            User::<Impl, OFFSET>,
            SetState::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            SetMeasurementPeriod::<Impl, OFFSET>,
            MeasurementPeriod::<Impl, OFFSET>,
            OverallCallRate::<Impl, OFFSET>,
            NumberOfACDCalls::<Impl, OFFSET>,
            NumberOfIncomingCalls::<Impl, OFFSET>,
            NumberOfOutgoingCalls::<Impl, OFFSET>,
            TotalACDTalkTime::<Impl, OFFSET>,
            TotalACDCallTime::<Impl, OFFSET>,
            TotalWrapUpTime::<Impl, OFFSET>,
            AgentSessions::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentEventImpl: Sized + IDispatchImpl {
    fn Agent();
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAgentEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAgentEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentEventVtbl {
    pub const fn new<Impl: ITAgentEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAgentEventVtbl {
        unsafe extern "system" fn Agent<Impl: ITAgentEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Agent(::core::mem::transmute_copy(&ppagent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAgentEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAgentEvent>, base.5, Agent::<Impl, OFFSET>, Event::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentHandlerImpl: Sized + IDispatchImpl {
    fn Name();
    fn CreateAgent();
    fn CreateAgentWithID();
    fn EnumerateACDGroups();
    fn EnumerateUsableAddresses();
    fn ACDGroups();
    fn UsableAddresses();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAgentHandler {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAgentHandler";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentHandlerVtbl {
    pub const fn new<Impl: ITAgentHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAgentHandlerVtbl {
        unsafe extern "system" fn Name<Impl: ITAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgent<Impl: ITAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAgent(::core::mem::transmute_copy(&ppagent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAgentWithID<Impl: ITAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAgentWithID(&*(&pid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&ppin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppagent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateACDGroups<Impl: ITAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateACDGroups(::core::mem::transmute_copy(&ppenumacdgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateUsableAddresses<Impl: ITAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateUsableAddresses(::core::mem::transmute_copy(&ppenumaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroups<Impl: ITAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ACDGroups(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsableAddresses<Impl: ITAgentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UsableAddresses(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAgentHandler>, base.5, Name::<Impl, OFFSET>, CreateAgent::<Impl, OFFSET>, CreateAgentWithID::<Impl, OFFSET>, EnumerateACDGroups::<Impl, OFFSET>, EnumerateUsableAddresses::<Impl, OFFSET>, ACDGroups::<Impl, OFFSET>, UsableAddresses::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentHandlerEventImpl: Sized + IDispatchImpl {
    fn AgentHandler();
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAgentHandlerEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAgentHandlerEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentHandlerEventVtbl {
    pub const fn new<Impl: ITAgentHandlerEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAgentHandlerEventVtbl {
        unsafe extern "system" fn AgentHandler<Impl: ITAgentHandlerEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppagenthandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AgentHandler(::core::mem::transmute_copy(&ppagenthandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAgentHandlerEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENTHANDLER_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAgentHandlerEvent>, base.5, AgentHandler::<Impl, OFFSET>, Event::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentSessionImpl: Sized + IDispatchImpl {
    fn Agent();
    fn Address();
    fn ACDGroup();
    fn SetState();
    fn State();
    fn SessionStartTime();
    fn SessionDuration();
    fn NumberOfCalls();
    fn TotalTalkTime();
    fn AverageTalkTime();
    fn TotalCallTime();
    fn AverageCallTime();
    fn TotalWrapUpTime();
    fn AverageWrapUpTime();
    fn ACDCallRate();
    fn LongestTimeToAnswer();
    fn AverageTimeToAnswer();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAgentSession {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAgentSession";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentSessionVtbl {
    pub const fn new<Impl: ITAgentSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAgentSessionVtbl {
        unsafe extern "system" fn Agent<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppagent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Agent(::core::mem::transmute_copy(&ppagent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address(::core::mem::transmute_copy(&ppaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDGroup<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppacdgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ACDGroup(::core::mem::transmute_copy(&ppacdgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionstate: AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetState(sessionstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psessionstate: *mut AGENT_SESSION_STATE) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SessionStartTime<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatesessionstart: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionStartTime(::core::mem::transmute_copy(&pdatesessionstart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionDuration<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionDuration(::core::mem::transmute_copy(&plduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfCalls<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberOfCalls(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTalkTime<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalTalkTime(::core::mem::transmute_copy(&pltalktime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTalkTime<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AverageTalkTime(::core::mem::transmute_copy(&pltalktime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallTime<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalCallTime(::core::mem::transmute_copy(&plcalltime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageCallTime<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AverageCallTime(::core::mem::transmute_copy(&plcalltime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWrapUpTime<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalWrapUpTime(::core::mem::transmute_copy(&plwrapuptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWrapUpTime<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AverageWrapUpTime(::core::mem::transmute_copy(&plwrapuptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ACDCallRate<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ACDCallRate(::core::mem::transmute_copy(&pcycallrate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestTimeToAnswer<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LongestTimeToAnswer(::core::mem::transmute_copy(&planswertime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageTimeToAnswer<Impl: ITAgentSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AverageTimeToAnswer(::core::mem::transmute_copy(&planswertime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITAgentSession>,
            base.5,
            Agent::<Impl, OFFSET>,
            Address::<Impl, OFFSET>,
            ACDGroup::<Impl, OFFSET>,
            SetState::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            SessionStartTime::<Impl, OFFSET>,
            SessionDuration::<Impl, OFFSET>,
            NumberOfCalls::<Impl, OFFSET>,
            TotalTalkTime::<Impl, OFFSET>,
            AverageTalkTime::<Impl, OFFSET>,
            TotalCallTime::<Impl, OFFSET>,
            AverageCallTime::<Impl, OFFSET>,
            TotalWrapUpTime::<Impl, OFFSET>,
            AverageWrapUpTime::<Impl, OFFSET>,
            ACDCallRate::<Impl, OFFSET>,
            LongestTimeToAnswer::<Impl, OFFSET>,
            AverageTimeToAnswer::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAgentSessionEventImpl: Sized + IDispatchImpl {
    fn Session();
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAgentSessionEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAgentSessionEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAgentSessionEventVtbl {
    pub const fn new<Impl: ITAgentSessionEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAgentSessionEventVtbl {
        unsafe extern "system" fn Session<Impl: ITAgentSessionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Session(::core::mem::transmute_copy(&ppsession)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITAgentSessionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_SESSION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAgentSessionEvent>, base.5, Session::<Impl, OFFSET>, Event::<Impl, OFFSET>)
    }
}
pub trait ITAllocatorPropertiesImpl: Sized {
    fn SetAllocatorProperties();
    fn GetAllocatorProperties();
    fn SetAllocateBuffers();
    fn GetAllocateBuffers();
    fn SetBufferSize();
    fn GetBufferSize();
}
impl ::windows::core::RuntimeName for ITAllocatorProperties {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAllocatorProperties";
}
impl ITAllocatorPropertiesVtbl {
    pub const fn new<Impl: ITAllocatorPropertiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAllocatorPropertiesVtbl {
        unsafe extern "system" fn SetAllocatorProperties<Impl: ITAllocatorPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllocatorProperties(&*(&pallocproperties as *const <super::super::Media::DirectShow::ALLOCATOR_PROPERTIES as ::windows::core::Abi>::Abi as *const <super::super::Media::DirectShow::ALLOCATOR_PROPERTIES as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllocatorProperties<Impl: ITAllocatorPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllocatorProperties(::core::mem::transmute_copy(&pallocproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateBuffers<Impl: ITAllocatorPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ballocbuffers: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllocateBuffers(&*(&ballocbuffers as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllocateBuffers<Impl: ITAllocatorPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pballocbuffers: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllocateBuffers(::core::mem::transmute_copy(&pballocbuffers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Impl: ITAllocatorPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBufferSize(buffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferSize<Impl: ITAllocatorPropertiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffersize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBufferSize(::core::mem::transmute_copy(&pbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITAllocatorProperties>, base.5, SetAllocatorProperties::<Impl, OFFSET>, GetAllocatorProperties::<Impl, OFFSET>, SetAllocateBuffers::<Impl, OFFSET>, GetAllocateBuffers::<Impl, OFFSET>, SetBufferSize::<Impl, OFFSET>, GetBufferSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITAutomatedPhoneControlImpl: Sized + IDispatchImpl {
    fn StartTone();
    fn StopTone();
    fn Tone();
    fn StartRinger();
    fn StopRinger();
    fn Ringer();
    fn SetPhoneHandlingEnabled();
    fn PhoneHandlingEnabled();
    fn SetAutoEndOfNumberTimeout();
    fn AutoEndOfNumberTimeout();
    fn SetAutoDialtone();
    fn AutoDialtone();
    fn SetAutoStopTonesOnOnHook();
    fn AutoStopTonesOnOnHook();
    fn SetAutoStopRingOnOffHook();
    fn AutoStopRingOnOffHook();
    fn SetAutoKeypadTones();
    fn AutoKeypadTones();
    fn SetAutoKeypadTonesMinimumDuration();
    fn AutoKeypadTonesMinimumDuration();
    fn SetAutoVolumeControl();
    fn AutoVolumeControl();
    fn SetAutoVolumeControlStep();
    fn AutoVolumeControlStep();
    fn SetAutoVolumeControlRepeatDelay();
    fn AutoVolumeControlRepeatDelay();
    fn SetAutoVolumeControlRepeatPeriod();
    fn AutoVolumeControlRepeatPeriod();
    fn SelectCall();
    fn UnselectCall();
    fn EnumerateSelectedCalls();
    fn SelectedCalls();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITAutomatedPhoneControl {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITAutomatedPhoneControl";
}
#[cfg(feature = "Win32_System_Com")]
impl ITAutomatedPhoneControlVtbl {
    pub const fn new<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITAutomatedPhoneControlVtbl {
        unsafe extern "system" fn StartTone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tone: PHONE_TONE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartTone(tone, lduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopTone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopTone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptone: *mut PHONE_TONE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tone(::core::mem::transmute_copy(&ptone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRinger<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lringmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartRinger(lringmode, lduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopRinger<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopRinger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ringer<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfringing: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ringer(::core::mem::transmute_copy(&pfringing)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhoneHandlingEnabled<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPhoneHandlingEnabled(fenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneHandlingEnabled<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhoneHandlingEnabled(::core::mem::transmute_copy(&pfenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoEndOfNumberTimeout<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoEndOfNumberTimeout(ltimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoEndOfNumberTimeout<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltimeout: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoEndOfNumberTimeout(::core::mem::transmute_copy(&pltimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoDialtone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoDialtone(fenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoDialtone<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoDialtone(::core::mem::transmute_copy(&pfenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopTonesOnOnHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoStopTonesOnOnHook(fenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoStopTonesOnOnHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoStopTonesOnOnHook(::core::mem::transmute_copy(&pfenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopRingOnOffHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoStopRingOnOffHook(fenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoStopRingOnOffHook<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoStopRingOnOffHook(::core::mem::transmute_copy(&pfenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTones<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoKeypadTones(fenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoKeypadTones<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoKeypadTones(::core::mem::transmute_copy(&pfenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoKeypadTonesMinimumDuration<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoKeypadTonesMinimumDuration(lduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoKeypadTonesMinimumDuration<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoKeypadTonesMinimumDuration(::core::mem::transmute_copy(&plduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControl<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoVolumeControl(fenabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoVolumeControl<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControl(::core::mem::transmute_copy(&fenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlStep<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstepsize: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoVolumeControlStep(lstepsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoVolumeControlStep<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstepsize: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControlStep(::core::mem::transmute_copy(&plstepsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatDelay<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoVolumeControlRepeatDelay(ldelay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoVolumeControlRepeatDelay<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControlRepeatDelay(::core::mem::transmute_copy(&pldelay)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatPeriod<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAutoVolumeControlRepeatPeriod(lperiod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoVolumeControlRepeatPeriod<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoVolumeControlRepeatPeriod(::core::mem::transmute_copy(&plperiod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectCall<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fselectdefaultterminals: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectCall(&*(&pcall as *const <ITCallInfo as ::windows::core::Abi>::Abi as *const <ITCallInfo as ::windows::core::DefaultType>::DefaultType), fselectdefaultterminals) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnselectCall<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnselectCall(&*(&pcall as *const <ITCallInfo as ::windows::core::Abi>::Abi as *const <ITCallInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSelectedCalls<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateSelectedCalls(::core::mem::transmute_copy(&ppcallenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedCalls<Impl: ITAutomatedPhoneControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedCalls(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITAutomatedPhoneControl>,
            base.5,
            StartTone::<Impl, OFFSET>,
            StopTone::<Impl, OFFSET>,
            Tone::<Impl, OFFSET>,
            StartRinger::<Impl, OFFSET>,
            StopRinger::<Impl, OFFSET>,
            Ringer::<Impl, OFFSET>,
            SetPhoneHandlingEnabled::<Impl, OFFSET>,
            PhoneHandlingEnabled::<Impl, OFFSET>,
            SetAutoEndOfNumberTimeout::<Impl, OFFSET>,
            AutoEndOfNumberTimeout::<Impl, OFFSET>,
            SetAutoDialtone::<Impl, OFFSET>,
            AutoDialtone::<Impl, OFFSET>,
            SetAutoStopTonesOnOnHook::<Impl, OFFSET>,
            AutoStopTonesOnOnHook::<Impl, OFFSET>,
            SetAutoStopRingOnOffHook::<Impl, OFFSET>,
            AutoStopRingOnOffHook::<Impl, OFFSET>,
            SetAutoKeypadTones::<Impl, OFFSET>,
            AutoKeypadTones::<Impl, OFFSET>,
            SetAutoKeypadTonesMinimumDuration::<Impl, OFFSET>,
            AutoKeypadTonesMinimumDuration::<Impl, OFFSET>,
            SetAutoVolumeControl::<Impl, OFFSET>,
            AutoVolumeControl::<Impl, OFFSET>,
            SetAutoVolumeControlStep::<Impl, OFFSET>,
            AutoVolumeControlStep::<Impl, OFFSET>,
            SetAutoVolumeControlRepeatDelay::<Impl, OFFSET>,
            AutoVolumeControlRepeatDelay::<Impl, OFFSET>,
            SetAutoVolumeControlRepeatPeriod::<Impl, OFFSET>,
            AutoVolumeControlRepeatPeriod::<Impl, OFFSET>,
            SelectCall::<Impl, OFFSET>,
            UnselectCall::<Impl, OFFSET>,
            EnumerateSelectedCalls::<Impl, OFFSET>,
            SelectedCalls::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITBasicAudioTerminalImpl: Sized + IDispatchImpl {
    fn SetVolume();
    fn Volume();
    fn SetBalance();
    fn Balance();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITBasicAudioTerminal {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITBasicAudioTerminal";
}
#[cfg(feature = "Win32_System_Com")]
impl ITBasicAudioTerminalVtbl {
    pub const fn new<Impl: ITBasicAudioTerminalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITBasicAudioTerminalVtbl {
        unsafe extern "system" fn SetVolume<Impl: ITBasicAudioTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVolume(lvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Impl: ITBasicAudioTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Volume(::core::mem::transmute_copy(&plvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBalance<Impl: ITBasicAudioTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBalance(lbalance) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Balance<Impl: ITBasicAudioTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Balance(::core::mem::transmute_copy(&plbalance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITBasicAudioTerminal>, base.5, SetVolume::<Impl, OFFSET>, Volume::<Impl, OFFSET>, SetBalance::<Impl, OFFSET>, Balance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITBasicCallControlImpl: Sized + IDispatchImpl {
    fn Connect();
    fn Answer();
    fn Disconnect();
    fn Hold();
    fn HandoffDirect();
    fn HandoffIndirect();
    fn Conference();
    fn Transfer();
    fn BlindTransfer();
    fn SwapHold();
    fn ParkDirect();
    fn ParkIndirect();
    fn Unpark();
    fn SetQOS();
    fn Pickup();
    fn Dial();
    fn Finish();
    fn RemoveFromConference();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITBasicCallControl {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITBasicCallControl";
}
#[cfg(feature = "Win32_System_Com")]
impl ITBasicCallControlVtbl {
    pub const fn new<Impl: ITBasicCallControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITBasicCallControlVtbl {
        unsafe extern "system" fn Connect<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connect(fsync) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Answer<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Answer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, code: DISCONNECT_CODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disconnect(code) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hold<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fhold: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hold(fhold) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandoffDirect<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandoffDirect(&*(&papplicationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandoffIndirect<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HandoffIndirect(lmediatype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Conference<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Conference(&*(&pcall as *const <ITBasicCallControl as ::windows::core::Abi>::Abi as *const <ITBasicCallControl as ::windows::core::DefaultType>::DefaultType), fsync) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transfer<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr, fsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transfer(&*(&pcall as *const <ITBasicCallControl as ::windows::core::Abi>::Abi as *const <ITBasicCallControl as ::windows::core::DefaultType>::DefaultType), fsync) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlindTransfer<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BlindTransfer(&*(&pdestaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwapHold<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SwapHold(&*(&pcall as *const <ITBasicCallControl as ::windows::core::Abi>::Abi as *const <ITBasicCallControl as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParkDirect<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparkaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParkDirect(&*(&pparkaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParkIndirect<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnondiraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ParkIndirect(::core::mem::transmute_copy(&ppnondiraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unpark<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unpark() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQOS<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQOS(lmediatype, servicelevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pickup<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroupid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pickup(&*(&pgroupid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dial<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dial(&*(&pdestaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finishmode: FINISH_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish(finishmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFromConference<Impl: ITBasicCallControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveFromConference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITBasicCallControl>,
            base.5,
            Connect::<Impl, OFFSET>,
            Answer::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
            Hold::<Impl, OFFSET>,
            HandoffDirect::<Impl, OFFSET>,
            HandoffIndirect::<Impl, OFFSET>,
            Conference::<Impl, OFFSET>,
            Transfer::<Impl, OFFSET>,
            BlindTransfer::<Impl, OFFSET>,
            SwapHold::<Impl, OFFSET>,
            ParkDirect::<Impl, OFFSET>,
            ParkIndirect::<Impl, OFFSET>,
            Unpark::<Impl, OFFSET>,
            SetQOS::<Impl, OFFSET>,
            Pickup::<Impl, OFFSET>,
            Dial::<Impl, OFFSET>,
            Finish::<Impl, OFFSET>,
            RemoveFromConference::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITBasicCallControl2Impl: Sized + ITBasicCallControlImpl + IDispatchImpl {
    fn RequestTerminal();
    fn SelectTerminalOnCall();
    fn UnselectTerminalOnCall();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITBasicCallControl2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITBasicCallControl2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITBasicCallControl2Vtbl {
    pub const fn new<Impl: ITBasicCallControl2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITBasicCallControl2Vtbl {
        unsafe extern "system" fn RequestTerminal<Impl: ITBasicCallControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrterminalclassguid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestTerminal(&*(&bstrterminalclassguid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lmediatype, direction, ::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectTerminalOnCall<Impl: ITBasicCallControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectTerminalOnCall(&*(&pterminal as *const <ITTerminal as ::windows::core::Abi>::Abi as *const <ITTerminal as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnselectTerminalOnCall<Impl: ITBasicCallControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnselectTerminalOnCall(&*(&pterminal as *const <ITTerminal as ::windows::core::Abi>::Abi as *const <ITTerminal as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITBasicCallControl2>, base.5, RequestTerminal::<Impl, OFFSET>, SelectTerminalOnCall::<Impl, OFFSET>, UnselectTerminalOnCall::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallHubImpl: Sized + IDispatchImpl {
    fn Clear();
    fn EnumerateCalls();
    fn Calls();
    fn NumCalls();
    fn State();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCallHub {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCallHub";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallHubVtbl {
    pub const fn new<Impl: ITCallHubImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCallHubVtbl {
        unsafe extern "system" fn Clear<Impl: ITCallHubImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCalls<Impl: ITCallHubImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateCalls(::core::mem::transmute_copy(&ppenumcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Calls<Impl: ITCallHubImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcalls: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Calls(::core::mem::transmute_copy(&pcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumCalls<Impl: ITCallHubImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumCalls(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITCallHubImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut CALLHUB_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCallHub>, base.5, Clear::<Impl, OFFSET>, EnumerateCalls::<Impl, OFFSET>, Calls::<Impl, OFFSET>, NumCalls::<Impl, OFFSET>, State::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallHubEventImpl: Sized + IDispatchImpl {
    fn Event();
    fn CallHub();
    fn Call();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCallHubEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCallHubEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallHubEventVtbl {
    pub const fn new<Impl: ITCallHubEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCallHubEventVtbl {
        unsafe extern "system" fn Event<Impl: ITCallHubEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut CALLHUB_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Impl: ITCallHubEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallHub(::core::mem::transmute_copy(&ppcallhub)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITCallHubEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCallHubEvent>, base.5, Event::<Impl, OFFSET>, CallHub::<Impl, OFFSET>, Call::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallInfoImpl: Sized + IDispatchImpl {
    fn Address();
    fn CallState();
    fn Privilege();
    fn CallHub();
    fn CallInfoLong();
    fn SetCallInfoLong();
    fn CallInfoString();
    fn SetCallInfoString();
    fn CallInfoBuffer();
    fn SetCallInfoBuffer();
    fn GetCallInfoBuffer();
    fn SetCallInfoBuffer();
    fn ReleaseUserUserInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCallInfo {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCallInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfoVtbl {
    pub const fn new<Impl: ITCallInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCallInfoVtbl {
        unsafe extern "system" fn Address<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address(::core::mem::transmute_copy(&ppaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallState<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallState(::core::mem::transmute_copy(&pcallstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprivilege: *mut CALL_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Privilege(::core::mem::transmute_copy(&pprivilege)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallHub(::core::mem::transmute_copy(&ppcallhub)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallInfoLong<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallInfoLong(callinfolong, ::core::mem::transmute_copy(&plcallinfolongval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoLong<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCallInfoLong(callinfolong, lcallinfolongval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallInfoString<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallInfoString(callinfostring, ::core::mem::transmute_copy(&ppcallinfostring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoString<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, pcallinfostring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCallInfoString(callinfostring, &*(&pcallinfostring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallInfoBuffer(callinfobuffer, ::core::mem::transmute_copy(&ppcallinfobuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCallInfoBuffer(callinfobuffer, &*(&pcallinfobuffer as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCallInfoBuffer(callinfobuffer, ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppcallinfobuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallInfoBuffer<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCallInfoBuffer(callinfobuffer, dwsize, pcallinfobuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseUserUserInfo<Impl: ITCallInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseUserUserInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITCallInfo>,
            base.5,
            Address::<Impl, OFFSET>,
            CallState::<Impl, OFFSET>,
            Privilege::<Impl, OFFSET>,
            CallHub::<Impl, OFFSET>,
            CallInfoLong::<Impl, OFFSET>,
            SetCallInfoLong::<Impl, OFFSET>,
            CallInfoString::<Impl, OFFSET>,
            SetCallInfoString::<Impl, OFFSET>,
            CallInfoBuffer::<Impl, OFFSET>,
            SetCallInfoBuffer::<Impl, OFFSET>,
            GetCallInfoBuffer::<Impl, OFFSET>,
            SetCallInfoBuffer::<Impl, OFFSET>,
            ReleaseUserUserInfo::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallInfo2Impl: Sized + ITCallInfoImpl + IDispatchImpl {
    fn EventFilter();
    fn SetEventFilter();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCallInfo2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCallInfo2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfo2Vtbl {
    pub const fn new<Impl: ITCallInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCallInfo2Vtbl {
        unsafe extern "system" fn EventFilter<Impl: ITCallInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EventFilter(tapievent, lsubevent, ::core::mem::transmute_copy(&penable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITCallInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEventFilter(tapievent, lsubevent, benable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCallInfo2>, base.5, EventFilter::<Impl, OFFSET>, SetEventFilter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallInfoChangeEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Cause();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCallInfoChangeEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCallInfoChangeEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfoChangeEventVtbl {
    pub const fn new<Impl: ITCallInfoChangeEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCallInfoChangeEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallInfoChangeEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITCallInfoChangeEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcic: *mut CALLINFOCHANGE_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cause(::core::mem::transmute_copy(&pcic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallInfoChangeEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallbackInstance(::core::mem::transmute_copy(&plcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCallInfoChangeEvent>, base.5, Call::<Impl, OFFSET>, Cause::<Impl, OFFSET>, CallbackInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallMediaEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Event();
    fn Error();
    fn Terminal();
    fn Stream();
    fn Cause();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCallMediaEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCallMediaEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallMediaEventVtbl {
    pub const fn new<Impl: ITCallMediaEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCallMediaEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallMediaEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcallinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITCallMediaEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pcallmediaevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITCallMediaEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error(::core::mem::transmute_copy(&phrerror)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminal<Impl: ITCallMediaEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminal(::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: ITCallMediaEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stream(::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITCallMediaEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cause(::core::mem::transmute_copy(&pcause)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCallMediaEvent>, base.5, Call::<Impl, OFFSET>, Event::<Impl, OFFSET>, Error::<Impl, OFFSET>, Terminal::<Impl, OFFSET>, Stream::<Impl, OFFSET>, Cause::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallNotificationEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Event();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCallNotificationEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCallNotificationEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallNotificationEventVtbl {
    pub const fn new<Impl: ITCallNotificationEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCallNotificationEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallNotificationEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITCallNotificationEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pcallnotificationevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallNotificationEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallbackInstance(::core::mem::transmute_copy(&plcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCallNotificationEvent>, base.5, Call::<Impl, OFFSET>, Event::<Impl, OFFSET>, CallbackInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallStateEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn State();
    fn Cause();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCallStateEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCallStateEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallStateEventVtbl {
    pub const fn new<Impl: ITCallStateEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCallStateEventVtbl {
        unsafe extern "system" fn Call<Impl: ITCallStateEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcallinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITCallStateEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pcallstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITCallStateEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcec: *mut CALL_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cause(::core::mem::transmute_copy(&pcec)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITCallStateEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallbackInstance(::core::mem::transmute_copy(&plcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCallStateEvent>, base.5, Call::<Impl, OFFSET>, State::<Impl, OFFSET>, Cause::<Impl, OFFSET>, CallbackInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCallingCardImpl: Sized + IDispatchImpl {
    fn PermanentCardID();
    fn NumberOfDigits();
    fn Options();
    fn CardName();
    fn SameAreaDialingRule();
    fn LongDistanceDialingRule();
    fn InternationalDialingRule();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCallingCard {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCallingCard";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallingCardVtbl {
    pub const fn new<Impl: ITCallingCardImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCallingCardVtbl {
        unsafe extern "system" fn PermanentCardID<Impl: ITCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PermanentCardID(::core::mem::transmute_copy(&plcardid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfDigits<Impl: ITCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldigits: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberOfDigits(::core::mem::transmute_copy(&pldigits)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: ITCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Options(::core::mem::transmute_copy(&ploptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CardName<Impl: ITCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcardname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CardName(::core::mem::transmute_copy(&ppcardname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SameAreaDialingRule<Impl: ITCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SameAreaDialingRule(::core::mem::transmute_copy(&pprule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceDialingRule<Impl: ITCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LongDistanceDialingRule(::core::mem::transmute_copy(&pprule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InternationalDialingRule<Impl: ITCallingCardImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprule: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InternationalDialingRule(::core::mem::transmute_copy(&pprule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCallingCard>, base.5, PermanentCardID::<Impl, OFFSET>, NumberOfDigits::<Impl, OFFSET>, Options::<Impl, OFFSET>, CardName::<Impl, OFFSET>, SameAreaDialingRule::<Impl, OFFSET>, LongDistanceDialingRule::<Impl, OFFSET>, InternationalDialingRule::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCollection {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCollectionVtbl {
    pub const fn new<Impl: ITCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ITCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&lcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ITCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ITCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppnewenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCollection>, base.5, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCollection2Impl: Sized + ITCollectionImpl + IDispatchImpl {
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCollection2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCollection2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCollection2Vtbl {
    pub const fn new<Impl: ITCollection2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCollection2Vtbl {
        unsafe extern "system" fn Add<Impl: ITCollection2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(index, &*(&pvariant as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ITCollection2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Remove(index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCollection2>, base.5, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITCustomToneImpl: Sized + IDispatchImpl {
    fn Frequency();
    fn SetFrequency();
    fn CadenceOn();
    fn SetCadenceOn();
    fn CadenceOff();
    fn SetCadenceOff();
    fn Volume();
    fn SetVolume();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITCustomTone {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITCustomTone";
}
#[cfg(feature = "Win32_System_Com")]
impl ITCustomToneVtbl {
    pub const fn new<Impl: ITCustomToneImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITCustomToneVtbl {
        unsafe extern "system" fn Frequency<Impl: ITCustomToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Frequency(::core::mem::transmute_copy(&plfrequency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Impl: ITCustomToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfrequency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFrequency(lfrequency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CadenceOn<Impl: ITCustomToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcadenceon: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CadenceOn(::core::mem::transmute_copy(&plcadenceon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOn<Impl: ITCustomToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cadenceon: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCadenceOn(cadenceon) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CadenceOff<Impl: ITCustomToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcadenceoff: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CadenceOff(::core::mem::transmute_copy(&plcadenceoff)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCadenceOff<Impl: ITCustomToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcadenceoff: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCadenceOff(lcadenceoff) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Impl: ITCustomToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Volume(::core::mem::transmute_copy(&plvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: ITCustomToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVolume(lvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITCustomTone>, base.5, Frequency::<Impl, OFFSET>, SetFrequency::<Impl, OFFSET>, CadenceOn::<Impl, OFFSET>, SetCadenceOn::<Impl, OFFSET>, CadenceOff::<Impl, OFFSET>, SetCadenceOff::<Impl, OFFSET>, Volume::<Impl, OFFSET>, SetVolume::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDetectToneImpl: Sized + IDispatchImpl {
    fn AppSpecific();
    fn SetAppSpecific();
    fn Duration();
    fn SetDuration();
    fn Frequency();
    fn SetFrequency();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITDetectTone {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITDetectTone";
}
#[cfg(feature = "Win32_System_Com")]
impl ITDetectToneVtbl {
    pub const fn new<Impl: ITDetectToneImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITDetectToneVtbl {
        unsafe extern "system" fn AppSpecific<Impl: ITDetectToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppSpecific(::core::mem::transmute_copy(&plappspecific)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppSpecific<Impl: ITDetectToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAppSpecific(lappspecific) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: ITDetectToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration(::core::mem::transmute_copy(&plduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: ITDetectToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDuration(lduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Frequency<Impl: ITDetectToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, plfrequency: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Frequency(index, ::core::mem::transmute_copy(&plfrequency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Impl: ITDetectToneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, lfrequency: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFrequency(index, lfrequency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITDetectTone>, base.5, AppSpecific::<Impl, OFFSET>, SetAppSpecific::<Impl, OFFSET>, Duration::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>, Frequency::<Impl, OFFSET>, SetFrequency::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDigitDetectionEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Digit();
    fn DigitMode();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITDigitDetectionEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITDigitDetectionEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITDigitDetectionEventVtbl {
    pub const fn new<Impl: ITDigitDetectionEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITDigitDetectionEventVtbl {
        unsafe extern "system" fn Call<Impl: ITDigitDetectionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcallinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digit<Impl: ITDigitDetectionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucdigit: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Digit(::core::mem::transmute_copy(&pucdigit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitMode<Impl: ITDigitDetectionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdigitmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DigitMode(::core::mem::transmute_copy(&pdigitmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitDetectionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TickCount(::core::mem::transmute_copy(&pltickcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitDetectionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallbackInstance(::core::mem::transmute_copy(&plcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITDigitDetectionEvent>, base.5, Call::<Impl, OFFSET>, Digit::<Impl, OFFSET>, DigitMode::<Impl, OFFSET>, TickCount::<Impl, OFFSET>, CallbackInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDigitGenerationEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn GenerationTermination();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITDigitGenerationEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITDigitGenerationEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITDigitGenerationEventVtbl {
    pub const fn new<Impl: ITDigitGenerationEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITDigitGenerationEventVtbl {
        unsafe extern "system" fn Call<Impl: ITDigitGenerationEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcallinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerationTermination<Impl: ITDigitGenerationEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plgenerationtermination: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerationTermination(::core::mem::transmute_copy(&plgenerationtermination)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitGenerationEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TickCount(::core::mem::transmute_copy(&pltickcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitGenerationEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallbackInstance(::core::mem::transmute_copy(&plcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITDigitGenerationEvent>, base.5, Call::<Impl, OFFSET>, GenerationTermination::<Impl, OFFSET>, TickCount::<Impl, OFFSET>, CallbackInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDigitsGatheredEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Digits();
    fn GatherTermination();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITDigitsGatheredEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITDigitsGatheredEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITDigitsGatheredEventVtbl {
    pub const fn new<Impl: ITDigitsGatheredEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITDigitsGatheredEventVtbl {
        unsafe extern "system" fn Call<Impl: ITDigitsGatheredEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcallinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Digits<Impl: ITDigitsGatheredEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdigits: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Digits(::core::mem::transmute_copy(&ppdigits)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GatherTermination<Impl: ITDigitsGatheredEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgathertermination: *mut TAPI_GATHERTERM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GatherTermination(::core::mem::transmute_copy(&pgathertermination)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITDigitsGatheredEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TickCount(::core::mem::transmute_copy(&pltickcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITDigitsGatheredEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallbackInstance(::core::mem::transmute_copy(&plcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITDigitsGatheredEvent>, base.5, Call::<Impl, OFFSET>, Digits::<Impl, OFFSET>, GatherTermination::<Impl, OFFSET>, TickCount::<Impl, OFFSET>, CallbackInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryImpl: Sized + IDispatchImpl {
    fn DirectoryType();
    fn DisplayName();
    fn IsDynamic();
    fn DefaultObjectTTL();
    fn SetDefaultObjectTTL();
    fn EnableAutoRefresh();
    fn Connect();
    fn Bind();
    fn AddDirectoryObject();
    fn ModifyDirectoryObject();
    fn RefreshDirectoryObject();
    fn DeleteDirectoryObject();
    fn DirectoryObjects();
    fn EnumerateDirectoryObjects();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITDirectory {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITDirectory";
}
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryVtbl {
    pub const fn new<Impl: ITDirectoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITDirectoryVtbl {
        unsafe extern "system" fn DirectoryType<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectorytype: *mut DIRECTORY_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DirectoryType(::core::mem::transmute_copy(&pdirectorytype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName(::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDynamic<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdynamic: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDynamic(::core::mem::transmute_copy(&pfdynamic)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultObjectTTL<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultObjectTTL(::core::mem::transmute_copy(&pttl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultObjectTTL<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ttl: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDefaultObjectTTL(ttl) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAutoRefresh<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableAutoRefresh(fenable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsecure: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connect(fsecure) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bind<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdomainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bind(
                &*(&pdomainname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pusername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                lflags,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddDirectoryObject(&*(&pdirectoryobject as *const <ITDirectoryObject as ::windows::core::Abi>::Abi as *const <ITDirectoryObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifyDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModifyDirectoryObject(&*(&pdirectoryobject as *const <ITDirectoryObject as ::windows::core::Abi>::Abi as *const <ITDirectoryObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RefreshDirectoryObject(&*(&pdirectoryobject as *const <ITDirectoryObject as ::windows::core::Abi>::Abi as *const <ITDirectoryObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDirectoryObject<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectoryobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteDirectoryObject(&*(&pdirectoryobject as *const <ITDirectoryObject as ::windows::core::Abi>::Abi as *const <ITDirectoryObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryObjects<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DirectoryObjects(directoryobjecttype, &*(&pname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDirectoryObjects<Impl: ITDirectoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppenumobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateDirectoryObjects(directoryobjecttype, &*(&pname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenumobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITDirectory>,
            base.5,
            DirectoryType::<Impl, OFFSET>,
            DisplayName::<Impl, OFFSET>,
            IsDynamic::<Impl, OFFSET>,
            DefaultObjectTTL::<Impl, OFFSET>,
            SetDefaultObjectTTL::<Impl, OFFSET>,
            EnableAutoRefresh::<Impl, OFFSET>,
            Connect::<Impl, OFFSET>,
            Bind::<Impl, OFFSET>,
            AddDirectoryObject::<Impl, OFFSET>,
            ModifyDirectoryObject::<Impl, OFFSET>,
            RefreshDirectoryObject::<Impl, OFFSET>,
            DeleteDirectoryObject::<Impl, OFFSET>,
            DirectoryObjects::<Impl, OFFSET>,
            EnumerateDirectoryObjects::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryObjectImpl: Sized + IDispatchImpl {
    fn ObjectType();
    fn Name();
    fn SetName();
    fn DialableAddrs();
    fn EnumerateDialableAddrs();
    fn SecurityDescriptor();
    fn SetSecurityDescriptor();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITDirectoryObject {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITDirectoryObject";
}
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObjectVtbl {
    pub const fn new<Impl: ITDirectoryObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITDirectoryObjectVtbl {
        unsafe extern "system" fn ObjectType<Impl: ITDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ObjectType(::core::mem::transmute_copy(&pobjecttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: ITDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&pname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DialableAddrs<Impl: ITDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaddresstype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DialableAddrs(dwaddresstype, ::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDialableAddrs<Impl: ITDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaddresstype: u32, ppenumdialableaddrs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateDialableAddrs(dwaddresstype, ::core::mem::transmute_copy(&ppenumdialableaddrs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecurityDescriptor<Impl: ITDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecdes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SecurityDescriptor(::core::mem::transmute_copy(&ppsecdes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Impl: ITDirectoryObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psecdes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSecurityDescriptor(&*(&psecdes as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITDirectoryObject>, base.5, ObjectType::<Impl, OFFSET>, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>, DialableAddrs::<Impl, OFFSET>, EnumerateDialableAddrs::<Impl, OFFSET>, SecurityDescriptor::<Impl, OFFSET>, SetSecurityDescriptor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryObjectConferenceImpl: Sized + IDispatchImpl {
    fn Protocol();
    fn Originator();
    fn SetOriginator();
    fn AdvertisingScope();
    fn SetAdvertisingScope();
    fn Url();
    fn SetUrl();
    fn Description();
    fn SetDescription();
    fn IsEncrypted();
    fn SetIsEncrypted();
    fn StartTime();
    fn SetStartTime();
    fn StopTime();
    fn SetStopTime();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITDirectoryObjectConference {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITDirectoryObjectConference";
}
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObjectConferenceVtbl {
    pub const fn new<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITDirectoryObjectConferenceVtbl {
        unsafe extern "system" fn Protocol<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprotocol: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Protocol(::core::mem::transmute_copy(&ppprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Originator<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pporiginator: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Originator(::core::mem::transmute_copy(&pporiginator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOriginator<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poriginator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOriginator(&*(&poriginator as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvertisingScope<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdvertisingScope(::core::mem::transmute_copy(&padvertisingscope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisingScope<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAdvertisingScope(advertisingscope) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Url<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Url(::core::mem::transmute_copy(&ppurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, purl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUrl(&*(&purl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&ppdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&pdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEncrypted<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfencrypted: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEncrypted(::core::mem::transmute_copy(&pfencrypted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEncrypted<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fencrypted: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIsEncrypted(fencrypted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartTime(::core::mem::transmute_copy(&pdate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStartTime(date) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopTime(::core::mem::transmute_copy(&pdate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStopTime<Impl: ITDirectoryObjectConferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStopTime(date) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITDirectoryObjectConference>,
            base.5,
            Protocol::<Impl, OFFSET>,
            Originator::<Impl, OFFSET>,
            SetOriginator::<Impl, OFFSET>,
            AdvertisingScope::<Impl, OFFSET>,
            SetAdvertisingScope::<Impl, OFFSET>,
            Url::<Impl, OFFSET>,
            SetUrl::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            IsEncrypted::<Impl, OFFSET>,
            SetIsEncrypted::<Impl, OFFSET>,
            StartTime::<Impl, OFFSET>,
            SetStartTime::<Impl, OFFSET>,
            StopTime::<Impl, OFFSET>,
            SetStopTime::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDirectoryObjectUserImpl: Sized + IDispatchImpl {
    fn IPPhonePrimary();
    fn SetIPPhonePrimary();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITDirectoryObjectUser {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITDirectoryObjectUser";
}
#[cfg(feature = "Win32_System_Com")]
impl ITDirectoryObjectUserVtbl {
    pub const fn new<Impl: ITDirectoryObjectUserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITDirectoryObjectUserVtbl {
        unsafe extern "system" fn IPPhonePrimary<Impl: ITDirectoryObjectUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IPPhonePrimary(::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIPPhonePrimary<Impl: ITDirectoryObjectUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetIPPhonePrimary(&*(&pname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITDirectoryObjectUser>, base.5, IPPhonePrimary::<Impl, OFFSET>, SetIPPhonePrimary::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITDispatchMapperImpl: Sized + IDispatchImpl {
    fn QueryDispatchInterface();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITDispatchMapper {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITDispatchMapper";
}
#[cfg(feature = "Win32_System_Com")]
impl ITDispatchMapperVtbl {
    pub const fn new<Impl: ITDispatchMapperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITDispatchMapperVtbl {
        unsafe extern "system" fn QueryDispatchInterface<Impl: ITDispatchMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pinterfacetomap: ::windows::core::RawPtr, ppreturnedinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryDispatchInterface(&*(&piid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pinterfacetomap as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppreturnedinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITDispatchMapper>, base.5, QueryDispatchInterface::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITFileTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Track();
    fn Call();
    fn State();
    fn Cause();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITFileTerminalEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITFileTerminalEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITFileTerminalEventVtbl {
    pub const fn new<Impl: ITFileTerminalEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITFileTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITFileTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminal(::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Track<Impl: ITFileTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptrackterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Track(::core::mem::transmute_copy(&pptrackterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITFileTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITFileTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cause<Impl: ITFileTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcause: *mut FT_STATE_EVENT_CAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cause(::core::mem::transmute_copy(&pcause)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITFileTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error(::core::mem::transmute_copy(&phrerrorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITFileTerminalEvent>, base.5, Terminal::<Impl, OFFSET>, Track::<Impl, OFFSET>, Call::<Impl, OFFSET>, State::<Impl, OFFSET>, Cause::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITFileTrackImpl: Sized + IDispatchImpl {
    fn Format();
    fn SetFormat();
    fn ControllingTerminal();
    fn AudioFormatForScripting();
    fn SetAudioFormatForScripting();
    fn EmptyAudioFormatForScripting();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITFileTrack {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITFileTrack";
}
#[cfg(feature = "Win32_System_Com")]
impl ITFileTrackVtbl {
    pub const fn new<Impl: ITFileTrackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITFileTrackVtbl {
        unsafe extern "system" fn Format<Impl: ITFileTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Format(::core::mem::transmute_copy(&ppmt)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: ITFileTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFormat(&*(&pmt as *const <super::super::Media::DirectShow::AM_MEDIA_TYPE as ::windows::core::Abi>::Abi as *const <super::super::Media::DirectShow::AM_MEDIA_TYPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ControllingTerminal<Impl: ITFileTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontrollingterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ControllingTerminal(::core::mem::transmute_copy(&ppcontrollingterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioFormatForScripting<Impl: ITFileTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudioFormatForScripting(::core::mem::transmute_copy(&ppaudioformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioFormatForScripting<Impl: ITFileTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paudioformat: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAudioFormatForScripting(&*(&paudioformat as *const <ITScriptableAudioFormat as ::windows::core::Abi>::Abi as *const <ITScriptableAudioFormat as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmptyAudioFormatForScripting<Impl: ITFileTrackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EmptyAudioFormatForScripting(::core::mem::transmute_copy(&ppaudioformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITFileTrack>, base.5, Format::<Impl, OFFSET>, SetFormat::<Impl, OFFSET>, ControllingTerminal::<Impl, OFFSET>, AudioFormatForScripting::<Impl, OFFSET>, SetAudioFormatForScripting::<Impl, OFFSET>, EmptyAudioFormatForScripting::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITForwardInformationImpl: Sized + IDispatchImpl {
    fn SetNumRingsNoAnswer();
    fn NumRingsNoAnswer();
    fn SetForwardType();
    fn ForwardTypeDestination();
    fn ForwardTypeCaller();
    fn GetForwardType();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITForwardInformation {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITForwardInformation";
}
#[cfg(feature = "Win32_System_Com")]
impl ITForwardInformationVtbl {
    pub const fn new<Impl: ITForwardInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITForwardInformationVtbl {
        unsafe extern "system" fn SetNumRingsNoAnswer<Impl: ITForwardInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnumrings: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNumRingsNoAnswer(lnumrings) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumRingsNoAnswer<Impl: ITForwardInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plnumrings: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumRingsNoAnswer(::core::mem::transmute_copy(&plnumrings)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForwardType<Impl: ITForwardInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetForwardType(forwardtype, &*(&pdestaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcalleraddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeDestination<Impl: ITForwardInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForwardTypeDestination(forwardtype, ::core::mem::transmute_copy(&ppdestaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeCaller<Impl: ITForwardInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForwardTypeCaller(forwardtype, ::core::mem::transmute_copy(&ppcalleraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForwardType<Impl: ITForwardInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, ppcalleraddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForwardType(forwardtype, ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&ppcalleraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ITForwardInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITForwardInformation>, base.5, SetNumRingsNoAnswer::<Impl, OFFSET>, NumRingsNoAnswer::<Impl, OFFSET>, SetForwardType::<Impl, OFFSET>, ForwardTypeDestination::<Impl, OFFSET>, ForwardTypeCaller::<Impl, OFFSET>, GetForwardType::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITForwardInformation2Impl: Sized + ITForwardInformationImpl + IDispatchImpl {
    fn SetForwardType2();
    fn GetForwardType2();
    fn ForwardTypeDestinationAddressType();
    fn ForwardTypeCallerAddressType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITForwardInformation2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITForwardInformation2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITForwardInformation2Vtbl {
    pub const fn new<Impl: ITForwardInformation2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITForwardInformation2Vtbl {
        unsafe extern "system" fn SetForwardType2<Impl: ITForwardInformation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, destaddresstype: i32, pcalleraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, calleraddresstype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetForwardType2(forwardtype, &*(&pdestaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), destaddresstype, &*(&pcalleraddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), calleraddresstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForwardType2<Impl: ITForwardInformation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut super::super::Foundation::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut super::super::Foundation::BSTR, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForwardType2(forwardtype, ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&pdestaddresstype), ::core::mem::transmute_copy(&ppcalleraddress), ::core::mem::transmute_copy(&pcalleraddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeDestinationAddressType<Impl: ITForwardInformation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForwardTypeDestinationAddressType(forwardtype, ::core::mem::transmute_copy(&pdestaddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardTypeCallerAddressType<Impl: ITForwardInformation2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pcalleraddresstype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForwardTypeCallerAddressType(forwardtype, ::core::mem::transmute_copy(&pcalleraddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITForwardInformation2>, base.5, SetForwardType2::<Impl, OFFSET>, GetForwardType2::<Impl, OFFSET>, ForwardTypeDestinationAddressType::<Impl, OFFSET>, ForwardTypeCallerAddressType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITILSConfigImpl: Sized + IDispatchImpl {
    fn Port();
    fn SetPort();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITILSConfig {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITILSConfig";
}
#[cfg(feature = "Win32_System_Com")]
impl ITILSConfigVtbl {
    pub const fn new<Impl: ITILSConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITILSConfigVtbl {
        unsafe extern "system" fn Port<Impl: ITILSConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pport: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Port(::core::mem::transmute_copy(&pport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Impl: ITILSConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, port: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPort(port) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITILSConfig>, base.5, Port::<Impl, OFFSET>, SetPort::<Impl, OFFSET>)
    }
}
pub trait ITLegacyAddressMediaControlImpl: Sized {
    fn GetID();
    fn GetDevConfig();
    fn SetDevConfig();
}
impl ::windows::core::RuntimeName for ITLegacyAddressMediaControl {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITLegacyAddressMediaControl";
}
impl ITLegacyAddressMediaControlVtbl {
    pub const fn new<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITLegacyAddressMediaControlVtbl {
        unsafe extern "system" fn GetID<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetID(&*(&pdeviceclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevConfig<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevConfig(&*(&pdeviceclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDevConfig<Impl: ITLegacyAddressMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsize: u32, pdeviceconfig: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDevConfig(&*(&pdeviceclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwsize, pdeviceconfig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITLegacyAddressMediaControl>, base.5, GetID::<Impl, OFFSET>, GetDevConfig::<Impl, OFFSET>, SetDevConfig::<Impl, OFFSET>)
    }
}
pub trait ITLegacyAddressMediaControl2Impl: Sized + ITLegacyAddressMediaControlImpl {
    fn ConfigDialog();
    fn ConfigDialogEdit();
}
impl ::windows::core::RuntimeName for ITLegacyAddressMediaControl2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITLegacyAddressMediaControl2";
}
impl ITLegacyAddressMediaControl2Vtbl {
    pub const fn new<Impl: ITLegacyAddressMediaControl2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITLegacyAddressMediaControl2Vtbl {
        unsafe extern "system" fn ConfigDialog<Impl: ITLegacyAddressMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConfigDialog(&*(&hwndowner as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&pdeviceclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigDialogEdit<Impl: ITLegacyAddressMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConfigDialogEdit(&*(&hwndowner as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&pdeviceclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwsizein, pdeviceconfigin, ::core::mem::transmute_copy(&pdwsizeout), ::core::mem::transmute_copy(&ppdeviceconfigout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITLegacyAddressMediaControl2>, base.5, ConfigDialog::<Impl, OFFSET>, ConfigDialogEdit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLegacyCallMediaControlImpl: Sized + IDispatchImpl {
    fn DetectDigits();
    fn GenerateDigits();
    fn GetID();
    fn SetMediaType();
    fn MonitorMedia();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITLegacyCallMediaControl {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITLegacyCallMediaControl";
}
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyCallMediaControlVtbl {
    pub const fn new<Impl: ITLegacyCallMediaControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITLegacyCallMediaControlVtbl {
        unsafe extern "system" fn DetectDigits<Impl: ITLegacyCallMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectDigits(digitmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateDigits<Impl: ITLegacyCallMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateDigits(&*(&pdigits as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), digitmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetID<Impl: ITLegacyCallMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetID(&*(&pdeviceclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaType<Impl: ITLegacyCallMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMediaType(lmediatype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MonitorMedia<Impl: ITLegacyCallMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MonitorMedia(lmediatype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITLegacyCallMediaControl>, base.5, DetectDigits::<Impl, OFFSET>, GenerateDigits::<Impl, OFFSET>, GetID::<Impl, OFFSET>, SetMediaType::<Impl, OFFSET>, MonitorMedia::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLegacyCallMediaControl2Impl: Sized + ITLegacyCallMediaControlImpl + IDispatchImpl {
    fn GenerateDigits2();
    fn GatherDigits();
    fn DetectTones();
    fn DetectTonesByCollection();
    fn GenerateTone();
    fn GenerateCustomTones();
    fn GenerateCustomTonesByCollection();
    fn CreateDetectToneObject();
    fn CreateCustomToneObject();
    fn GetIDAsVariant();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITLegacyCallMediaControl2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITLegacyCallMediaControl2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyCallMediaControl2Vtbl {
    pub const fn new<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITLegacyCallMediaControl2Vtbl {
        unsafe extern "system" fn GenerateDigits2<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, digitmode: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateDigits2(&*(&pdigits as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), digitmode, lduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GatherDigits<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digitmode: i32, lnumdigits: i32, pterminationdigits: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GatherDigits(digitmode, lnumdigits, &*(&pterminationdigits as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lfirstdigittimeout, linterdigittimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectTones<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectTones(&*(&ptonelist as *const <TAPI_DETECTTONE as ::windows::core::Abi>::Abi as *const <TAPI_DETECTTONE as ::windows::core::DefaultType>::DefaultType), lnumtones) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectTonesByCollection<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdetecttonecollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DetectTonesByCollection(&*(&pdetecttonecollection as *const <ITCollection2 as ::windows::core::Abi>::Abi as *const <ITCollection2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateTone<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateTone(tonemode, lduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateCustomTones<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateCustomTones(&*(&ptonelist as *const <TAPI_CUSTOMTONE as ::windows::core::Abi>::Abi as *const <TAPI_CUSTOMTONE as ::windows::core::DefaultType>::DefaultType), lnumtones, lduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateCustomTonesByCollection<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcustomtonecollection: ::windows::core::RawPtr, lduration: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateCustomTonesByCollection(&*(&pcustomtonecollection as *const <ITCollection2 as ::windows::core::Abi>::Abi as *const <ITCollection2 as ::windows::core::DefaultType>::DefaultType), lduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDetectToneObject<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdetecttone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDetectToneObject(::core::mem::transmute_copy(&ppdetecttone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCustomToneObject<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcustomtone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCustomToneObject(::core::mem::transmute_copy(&ppcustomtone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDAsVariant<Impl: ITLegacyCallMediaControl2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardeviceid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIDAsVariant(&*(&bstrdeviceclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvardeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITLegacyCallMediaControl2>, base.5, GenerateDigits2::<Impl, OFFSET>, GatherDigits::<Impl, OFFSET>, DetectTones::<Impl, OFFSET>, DetectTonesByCollection::<Impl, OFFSET>, GenerateTone::<Impl, OFFSET>, GenerateCustomTones::<Impl, OFFSET>, GenerateCustomTonesByCollection::<Impl, OFFSET>, CreateDetectToneObject::<Impl, OFFSET>, CreateCustomToneObject::<Impl, OFFSET>, GetIDAsVariant::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLegacyWaveSupportImpl: Sized + IDispatchImpl {
    fn IsFullDuplex();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITLegacyWaveSupport {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITLegacyWaveSupport";
}
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyWaveSupportVtbl {
    pub const fn new<Impl: ITLegacyWaveSupportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITLegacyWaveSupportVtbl {
        unsafe extern "system" fn IsFullDuplex<Impl: ITLegacyWaveSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupport: *mut FULLDUPLEX_SUPPORT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsFullDuplex(::core::mem::transmute_copy(&psupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITLegacyWaveSupport>, base.5, IsFullDuplex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITLocationInfoImpl: Sized + IDispatchImpl {
    fn PermanentLocationID();
    fn CountryCode();
    fn CountryID();
    fn Options();
    fn PreferredCardID();
    fn LocationName();
    fn CityCode();
    fn LocalAccessCode();
    fn LongDistanceAccessCode();
    fn TollPrefixList();
    fn CancelCallWaitingCode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITLocationInfo {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITLocationInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl ITLocationInfoVtbl {
    pub const fn new<Impl: ITLocationInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITLocationInfoVtbl {
        unsafe extern "system" fn PermanentLocationID<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllocationid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PermanentLocationID(::core::mem::transmute_copy(&pllocationid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryCode<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CountryCode(::core::mem::transmute_copy(&plcountrycode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CountryID<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcountryid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CountryID(::core::mem::transmute_copy(&plcountryid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Options(::core::mem::transmute_copy(&ploptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredCardID<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredCardID(::core::mem::transmute_copy(&plcardid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationName<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplocationname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationName(::core::mem::transmute_copy(&pplocationname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CityCode<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CityCode(::core::mem::transmute_copy(&ppcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAccessCode<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalAccessCode(::core::mem::transmute_copy(&ppcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongDistanceAccessCode<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LongDistanceAccessCode(::core::mem::transmute_copy(&ppcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TollPrefixList<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptolllist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TollPrefixList(::core::mem::transmute_copy(&pptolllist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelCallWaitingCode<Impl: ITLocationInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcode: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelCallWaitingCode(::core::mem::transmute_copy(&ppcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITLocationInfo>, base.5, PermanentLocationID::<Impl, OFFSET>, CountryCode::<Impl, OFFSET>, CountryID::<Impl, OFFSET>, Options::<Impl, OFFSET>, PreferredCardID::<Impl, OFFSET>, LocationName::<Impl, OFFSET>, CityCode::<Impl, OFFSET>, LocalAccessCode::<Impl, OFFSET>, LongDistanceAccessCode::<Impl, OFFSET>, TollPrefixList::<Impl, OFFSET>, CancelCallWaitingCode::<Impl, OFFSET>)
    }
}
pub trait ITMSPAddressImpl: Sized {
    fn Initialize();
    fn Shutdown();
    fn CreateMSPCall();
    fn ShutdownMSPCall();
    fn ReceiveTSPData();
    fn GetEvent();
}
impl ::windows::core::RuntimeName for ITMSPAddress {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITMSPAddress";
}
impl ITMSPAddressVtbl {
    pub const fn new<Impl: ITMSPAddressImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITMSPAddressVtbl {
        unsafe extern "system" fn Initialize<Impl: ITMSPAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(hevent) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: ITMSPAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateMSPCall<Impl: ITMSPAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut ::core::ffi::c_void, ppstreamcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateMSPCall(hcall, dwreserved, dwmediatype, &*(&pouterunknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppstreamcontrol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownMSPCall<Impl: ITMSPAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstreamcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShutdownMSPCall(&*(&pstreamcontrol as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveTSPData<Impl: ITMSPAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmspcall: *mut ::core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReceiveTSPData(&*(&pmspcall as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), pbuffer, dwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEvent<Impl: ITMSPAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetEvent(pdwsize, peventbuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITMSPAddress>, base.5, Initialize::<Impl, OFFSET>, Shutdown::<Impl, OFFSET>, CreateMSPCall::<Impl, OFFSET>, ShutdownMSPCall::<Impl, OFFSET>, ReceiveTSPData::<Impl, OFFSET>, GetEvent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaControlImpl: Sized + IDispatchImpl {
    fn Start();
    fn Stop();
    fn Pause();
    fn MediaState();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITMediaControl {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITMediaControl";
}
#[cfg(feature = "Win32_System_Com")]
impl ITMediaControlVtbl {
    pub const fn new<Impl: ITMediaControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITMediaControlVtbl {
        unsafe extern "system" fn Start<Impl: ITMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: ITMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Impl: ITMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pause() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaState<Impl: ITMediaControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaState(::core::mem::transmute_copy(&pterminalmediastate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITMediaControl>, base.5, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, Pause::<Impl, OFFSET>, MediaState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaPlaybackImpl: Sized + IDispatchImpl {
    fn SetPlayList();
    fn PlayList();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITMediaPlayback {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITMediaPlayback";
}
#[cfg(feature = "Win32_System_Com")]
impl ITMediaPlaybackVtbl {
    pub const fn new<Impl: ITMediaPlaybackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITMediaPlaybackVtbl {
        unsafe extern "system" fn SetPlayList<Impl: ITMediaPlaybackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, playlistvariant: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPlayList(&*(&playlistvariant as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlayList<Impl: ITMediaPlaybackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplaylistvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlayList(::core::mem::transmute_copy(&pplaylistvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITMediaPlayback>, base.5, SetPlayList::<Impl, OFFSET>, PlayList::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaRecordImpl: Sized + IDispatchImpl {
    fn SetFileName();
    fn FileName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITMediaRecord {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITMediaRecord";
}
#[cfg(feature = "Win32_System_Com")]
impl ITMediaRecordVtbl {
    pub const fn new<Impl: ITMediaRecordImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITMediaRecordVtbl {
        unsafe extern "system" fn SetFileName<Impl: ITMediaRecordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFileName(&*(&bstrfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileName<Impl: ITMediaRecordImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FileName(::core::mem::transmute_copy(&pbstrfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITMediaRecord>, base.5, SetFileName::<Impl, OFFSET>, FileName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMediaSupportImpl: Sized + IDispatchImpl {
    fn MediaTypes();
    fn QueryMediaType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITMediaSupport {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITMediaSupport";
}
#[cfg(feature = "Win32_System_Com")]
impl ITMediaSupportVtbl {
    pub const fn new<Impl: ITMediaSupportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITMediaSupportVtbl {
        unsafe extern "system" fn MediaTypes<Impl: ITMediaSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaTypes(::core::mem::transmute_copy(&plmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMediaType<Impl: ITMediaSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, pfsupport: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryMediaType(lmediatype, ::core::mem::transmute_copy(&pfsupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITMediaSupport>, base.5, MediaTypes::<Impl, OFFSET>, QueryMediaType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITMultiTrackTerminalImpl: Sized + IDispatchImpl {
    fn TrackTerminals();
    fn EnumerateTrackTerminals();
    fn CreateTrackTerminal();
    fn MediaTypesInUse();
    fn DirectionsInUse();
    fn RemoveTrackTerminal();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITMultiTrackTerminal {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITMultiTrackTerminal";
}
#[cfg(feature = "Win32_System_Com")]
impl ITMultiTrackTerminalVtbl {
    pub const fn new<Impl: ITMultiTrackTerminalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITMultiTrackTerminalVtbl {
        unsafe extern "system" fn TrackTerminals<Impl: ITMultiTrackTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrackTerminals(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTrackTerminals<Impl: ITMultiTrackTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateTrackTerminals(::core::mem::transmute_copy(&ppenumterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTrackTerminal<Impl: ITMultiTrackTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTrackTerminal(mediatype, terminaldirection, ::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypesInUse<Impl: ITMultiTrackTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatypesinuse: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaTypesInUse(::core::mem::transmute_copy(&plmediatypesinuse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectionsInUse<Impl: ITMultiTrackTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldirectionsinused: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DirectionsInUse(::core::mem::transmute_copy(&pldirectionsinused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTrackTerminal<Impl: ITMultiTrackTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrackterminaltoremove: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveTrackTerminal(&*(&ptrackterminaltoremove as *const <ITTerminal as ::windows::core::Abi>::Abi as *const <ITTerminal as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITMultiTrackTerminal>, base.5, TrackTerminals::<Impl, OFFSET>, EnumerateTrackTerminals::<Impl, OFFSET>, CreateTrackTerminal::<Impl, OFFSET>, MediaTypesInUse::<Impl, OFFSET>, DirectionsInUse::<Impl, OFFSET>, RemoveTrackTerminal::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPhoneImpl: Sized + IDispatchImpl {
    fn Open();
    fn Close();
    fn Addresses();
    fn EnumerateAddresses();
    fn PhoneCapsLong();
    fn PhoneCapsString();
    fn Terminals();
    fn EnumerateTerminals();
    fn ButtonMode();
    fn SetButtonMode();
    fn ButtonFunction();
    fn SetButtonFunction();
    fn ButtonText();
    fn SetButtonText();
    fn ButtonState();
    fn HookSwitchState();
    fn SetHookSwitchState();
    fn SetRingMode();
    fn RingMode();
    fn SetRingVolume();
    fn RingVolume();
    fn Privilege();
    fn GetPhoneCapsBuffer();
    fn PhoneCapsBuffer();
    fn LampMode();
    fn SetLampMode();
    fn Display();
    fn SetDisplay();
    fn PreferredAddresses();
    fn EnumeratePreferredAddresses();
    fn DeviceSpecific();
    fn DeviceSpecificVariant();
    fn NegotiateExtVersion();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITPhone {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITPhone";
}
#[cfg(feature = "Win32_System_Com")]
impl ITPhoneVtbl {
    pub const fn new<Impl: ITPhoneImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITPhoneVtbl {
        unsafe extern "system" fn Open<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, privilege: PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(privilege) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Addresses<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Addresses(::core::mem::transmute_copy(&paddresses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateAddresses(::core::mem::transmute_copy(&ppenumaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneCapsLong<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhoneCapsLong(pclcap, ::core::mem::transmute_copy(&plcapability)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneCapsString<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcscap: PHONECAPS_STRING, ppcapability: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhoneCapsString(pcscap, ::core::mem::transmute_copy(&ppcapability)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminals(&*(&paddress as *const <ITAddress as ::windows::core::Abi>::Abi as *const <ITAddress as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pterminals)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateTerminals(&*(&paddress as *const <ITAddress as ::windows::core::Abi>::Abi as *const <ITAddress as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenumterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonMode<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonMode(lbuttonid, ::core::mem::transmute_copy(&pbuttonmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonMode<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetButtonMode(lbuttonid, buttonmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonFunction<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonFunction(lbuttonid, ::core::mem::transmute_copy(&pbuttonfunction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonFunction<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetButtonFunction(lbuttonid, buttonfunction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonText<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, ppbuttontext: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonText(lbuttonid, ::core::mem::transmute_copy(&ppbuttontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonText<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, bstrbuttontext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetButtonText(lbuttonid, &*(&bstrbuttontext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonState<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonState(lbuttonid, ::core::mem::transmute_copy(&pbuttonstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HookSwitchState(hookswitchdevice, ::core::mem::transmute_copy(&phookswitchstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHookSwitchState<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHookSwitchState(hookswitchdevice, hookswitchstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingMode<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lringmode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRingMode(lringmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingMode<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RingMode(::core::mem::transmute_copy(&plringmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRingVolume<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lringvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRingVolume(lringvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingVolume<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plringvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RingVolume(::core::mem::transmute_copy(&plringvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Privilege<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprivilege: *mut PHONE_PRIVILEGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Privilege(::core::mem::transmute_copy(&pprivilege)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPhoneCapsBuffer<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPhoneCapsBuffer(pcbcaps, ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppphonecapsbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneCapsBuffer<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhoneCapsBuffer(pcbcaps, ::core::mem::transmute_copy(&pvarbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LampMode<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LampMode(llampid, ::core::mem::transmute_copy(&plampmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLampMode<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLampMode(llampid, lampmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Display<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdisplay: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Display(::core::mem::transmute_copy(&pbstrdisplay)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplay<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrow: i32, lcolumn: i32, bstrdisplay: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisplay(lrow, lcolumn, &*(&bstrdisplay as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredAddresses<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredAddresses(::core::mem::transmute_copy(&paddresses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePreferredAddresses<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumeratePreferredAddresses(::core::mem::transmute_copy(&ppenumaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSpecific<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceSpecific(pparams, dwsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSpecificVariant<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardevspecificbytearray: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceSpecificVariant(&*(&vardevspecificbytearray as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegotiateExtVersion<Impl: ITPhoneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegotiateExtVersion(llowversion, lhighversion, ::core::mem::transmute_copy(&plextversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITPhone>,
            base.5,
            Open::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Addresses::<Impl, OFFSET>,
            EnumerateAddresses::<Impl, OFFSET>,
            PhoneCapsLong::<Impl, OFFSET>,
            PhoneCapsString::<Impl, OFFSET>,
            Terminals::<Impl, OFFSET>,
            EnumerateTerminals::<Impl, OFFSET>,
            ButtonMode::<Impl, OFFSET>,
            SetButtonMode::<Impl, OFFSET>,
            ButtonFunction::<Impl, OFFSET>,
            SetButtonFunction::<Impl, OFFSET>,
            ButtonText::<Impl, OFFSET>,
            SetButtonText::<Impl, OFFSET>,
            ButtonState::<Impl, OFFSET>,
            HookSwitchState::<Impl, OFFSET>,
            SetHookSwitchState::<Impl, OFFSET>,
            SetRingMode::<Impl, OFFSET>,
            RingMode::<Impl, OFFSET>,
            SetRingVolume::<Impl, OFFSET>,
            RingVolume::<Impl, OFFSET>,
            Privilege::<Impl, OFFSET>,
            GetPhoneCapsBuffer::<Impl, OFFSET>,
            PhoneCapsBuffer::<Impl, OFFSET>,
            LampMode::<Impl, OFFSET>,
            SetLampMode::<Impl, OFFSET>,
            Display::<Impl, OFFSET>,
            SetDisplay::<Impl, OFFSET>,
            PreferredAddresses::<Impl, OFFSET>,
            EnumeratePreferredAddresses::<Impl, OFFSET>,
            DeviceSpecific::<Impl, OFFSET>,
            DeviceSpecificVariant::<Impl, OFFSET>,
            NegotiateExtVersion::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPhoneDeviceSpecificEventImpl: Sized + IDispatchImpl {
    fn Phone();
    fn lParam1();
    fn lParam2();
    fn lParam3();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITPhoneDeviceSpecificEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITPhoneDeviceSpecificEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITPhoneDeviceSpecificEventVtbl {
    pub const fn new<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITPhoneDeviceSpecificEventVtbl {
        unsafe extern "system" fn Phone<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Phone(::core::mem::transmute_copy(&ppphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam1<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).lParam1(::core::mem::transmute_copy(&pparam1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam2<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).lParam2(::core::mem::transmute_copy(&pparam2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lParam3<Impl: ITPhoneDeviceSpecificEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).lParam3(::core::mem::transmute_copy(&pparam3)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITPhoneDeviceSpecificEvent>, base.5, Phone::<Impl, OFFSET>, lParam1::<Impl, OFFSET>, lParam2::<Impl, OFFSET>, lParam3::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPhoneEventImpl: Sized + IDispatchImpl {
    fn Phone();
    fn Event();
    fn ButtonState();
    fn HookSwitchState();
    fn HookSwitchDevice();
    fn RingMode();
    fn ButtonLampId();
    fn NumberGathered();
    fn Call();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITPhoneEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITPhoneEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITPhoneEventVtbl {
    pub const fn new<Impl: ITPhoneEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITPhoneEventVtbl {
        unsafe extern "system" fn Phone<Impl: ITPhoneEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Phone(::core::mem::transmute_copy(&ppphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITPhoneEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut PHONE_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonState<Impl: ITPhoneEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_BUTTON_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonState(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchState<Impl: ITPhoneEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HookSwitchState(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HookSwitchDevice<Impl: ITPhoneEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HookSwitchDevice(::core::mem::transmute_copy(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingMode<Impl: ITPhoneEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RingMode(::core::mem::transmute_copy(&plringmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ButtonLampId<Impl: ITPhoneEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbuttonlampid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ButtonLampId(::core::mem::transmute_copy(&plbuttonlampid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberGathered<Impl: ITPhoneEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnumber: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NumberGathered(::core::mem::transmute_copy(&ppnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITPhoneEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcallinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITPhoneEvent>, base.5, Phone::<Impl, OFFSET>, Event::<Impl, OFFSET>, ButtonState::<Impl, OFFSET>, HookSwitchState::<Impl, OFFSET>, HookSwitchDevice::<Impl, OFFSET>, RingMode::<Impl, OFFSET>, ButtonLampId::<Impl, OFFSET>, NumberGathered::<Impl, OFFSET>, Call::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalClassInfoImpl: Sized + IDispatchImpl {
    fn Name();
    fn Company();
    fn Version();
    fn TerminalClass();
    fn CLSID();
    fn Direction();
    fn MediaTypes();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITPluggableTerminalClassInfo {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITPluggableTerminalClassInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalClassInfoVtbl {
    pub const fn new<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITPluggableTerminalClassInfoVtbl {
        unsafe extern "system" fn Name<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Company<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompany: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Company(::core::mem::transmute_copy(&pcompany)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Version(::core::mem::transmute_copy(&pversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TerminalClass(::core::mem::transmute_copy(&pterminalclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CLSID(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direction(::core::mem::transmute_copy(&pdirection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaTypes<Impl: ITPluggableTerminalClassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaTypes(::core::mem::transmute_copy(&pmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITPluggableTerminalClassInfo>, base.5, Name::<Impl, OFFSET>, Company::<Impl, OFFSET>, Version::<Impl, OFFSET>, TerminalClass::<Impl, OFFSET>, CLSID::<Impl, OFFSET>, Direction::<Impl, OFFSET>, MediaTypes::<Impl, OFFSET>)
    }
}
pub trait ITPluggableTerminalEventSinkImpl: Sized {
    fn FireEvent();
}
impl ::windows::core::RuntimeName for ITPluggableTerminalEventSink {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITPluggableTerminalEventSink";
}
impl ITPluggableTerminalEventSinkVtbl {
    pub const fn new<Impl: ITPluggableTerminalEventSinkImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITPluggableTerminalEventSinkVtbl {
        unsafe extern "system" fn FireEvent<Impl: ITPluggableTerminalEventSinkImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FireEvent(&*(&pmspeventinfo as *const <MSP_EVENT_INFO as ::windows::core::Abi>::Abi as *const <MSP_EVENT_INFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITPluggableTerminalEventSink>, base.5, FireEvent::<Impl, OFFSET>)
    }
}
pub trait ITPluggableTerminalEventSinkRegistrationImpl: Sized {
    fn RegisterSink();
    fn UnregisterSink();
}
impl ::windows::core::RuntimeName for ITPluggableTerminalEventSinkRegistration {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITPluggableTerminalEventSinkRegistration";
}
impl ITPluggableTerminalEventSinkRegistrationVtbl {
    pub const fn new<Impl: ITPluggableTerminalEventSinkRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITPluggableTerminalEventSinkRegistrationVtbl {
        unsafe extern "system" fn RegisterSink<Impl: ITPluggableTerminalEventSinkRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterSink(&*(&peventsink as *const <ITPluggableTerminalEventSink as ::windows::core::Abi>::Abi as *const <ITPluggableTerminalEventSink as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSink<Impl: ITPluggableTerminalEventSinkRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterSink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITPluggableTerminalEventSinkRegistration>, base.5, RegisterSink::<Impl, OFFSET>, UnregisterSink::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalSuperclassInfoImpl: Sized + IDispatchImpl {
    fn Name();
    fn CLSID();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITPluggableTerminalSuperclassInfo {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITPluggableTerminalSuperclassInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl ITPluggableTerminalSuperclassInfoVtbl {
    pub const fn new<Impl: ITPluggableTerminalSuperclassInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITPluggableTerminalSuperclassInfoVtbl {
        unsafe extern "system" fn Name<Impl: ITPluggableTerminalSuperclassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CLSID<Impl: ITPluggableTerminalSuperclassInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CLSID(::core::mem::transmute_copy(&pclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITPluggableTerminalSuperclassInfo>, base.5, Name::<Impl, OFFSET>, CLSID::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITPrivateEventImpl: Sized + IDispatchImpl {
    fn Address();
    fn Call();
    fn CallHub();
    fn EventCode();
    fn EventInterface();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITPrivateEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITPrivateEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITPrivateEventVtbl {
    pub const fn new<Impl: ITPrivateEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITPrivateEventVtbl {
        unsafe extern "system" fn Address<Impl: ITPrivateEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address(::core::mem::transmute_copy(&ppaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITPrivateEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcallinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHub<Impl: ITPrivateEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallHub(::core::mem::transmute_copy(&ppcallhub)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventCode<Impl: ITPrivateEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pleventcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EventCode(::core::mem::transmute_copy(&pleventcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterface<Impl: ITPrivateEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EventInterface(::core::mem::transmute_copy(&peventinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITPrivateEvent>, base.5, Address::<Impl, OFFSET>, Call::<Impl, OFFSET>, CallHub::<Impl, OFFSET>, EventCode::<Impl, OFFSET>, EventInterface::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITQOSEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn Event();
    fn MediaType();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITQOSEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITQOSEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITQOSEventVtbl {
    pub const fn new<Impl: ITQOSEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITQOSEventVtbl {
        unsafe extern "system" fn Call<Impl: ITQOSEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITQOSEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqosevent: *mut QOS_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pqosevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: ITQOSEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaType(::core::mem::transmute_copy(&plmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITQOSEvent>, base.5, Call::<Impl, OFFSET>, Event::<Impl, OFFSET>, MediaType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITQueueImpl: Sized + IDispatchImpl {
    fn SetMeasurementPeriod();
    fn MeasurementPeriod();
    fn TotalCallsQueued();
    fn CurrentCallsQueued();
    fn TotalCallsAbandoned();
    fn TotalCallsFlowedIn();
    fn TotalCallsFlowedOut();
    fn LongestEverWaitTime();
    fn CurrentLongestWaitTime();
    fn AverageWaitTime();
    fn FinalDisposition();
    fn Name();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITQueue {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITQueue";
}
#[cfg(feature = "Win32_System_Com")]
impl ITQueueVtbl {
    pub const fn new<Impl: ITQueueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITQueueVtbl {
        unsafe extern "system" fn SetMeasurementPeriod<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMeasurementPeriod(lperiod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasurementPeriod<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MeasurementPeriod(::core::mem::transmute_copy(&plperiod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsQueued<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalCallsQueued(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentCallsQueued<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentCallsQueued(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsAbandoned<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalCallsAbandoned(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedIn<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalCallsFlowedIn(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCallsFlowedOut<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalCallsFlowedOut(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongestEverWaitTime<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LongestEverWaitTime(::core::mem::transmute_copy(&plwaittime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentLongestWaitTime<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentLongestWaitTime(::core::mem::transmute_copy(&plwaittime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AverageWaitTime<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AverageWaitTime(::core::mem::transmute_copy(&plwaittime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalDisposition<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinalDisposition(::core::mem::transmute_copy(&plcalls)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITQueue>,
            base.5,
            SetMeasurementPeriod::<Impl, OFFSET>,
            MeasurementPeriod::<Impl, OFFSET>,
            TotalCallsQueued::<Impl, OFFSET>,
            CurrentCallsQueued::<Impl, OFFSET>,
            TotalCallsAbandoned::<Impl, OFFSET>,
            TotalCallsFlowedIn::<Impl, OFFSET>,
            TotalCallsFlowedOut::<Impl, OFFSET>,
            LongestEverWaitTime::<Impl, OFFSET>,
            CurrentLongestWaitTime::<Impl, OFFSET>,
            AverageWaitTime::<Impl, OFFSET>,
            FinalDisposition::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITQueueEventImpl: Sized + IDispatchImpl {
    fn Queue();
    fn Event();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITQueueEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITQueueEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITQueueEventVtbl {
    pub const fn new<Impl: ITQueueEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITQueueEventVtbl {
        unsafe extern "system" fn Queue<Impl: ITQueueEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqueue: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Queue(::core::mem::transmute_copy(&ppqueue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITQueueEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDQUEUE_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITQueueEvent>, base.5, Queue::<Impl, OFFSET>, Event::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITRendezvousImpl: Sized + IDispatchImpl {
    fn DefaultDirectories();
    fn EnumerateDefaultDirectories();
    fn CreateDirectory();
    fn CreateDirectoryObject();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITRendezvous {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITRendezvous";
}
#[cfg(feature = "Win32_System_Com")]
impl ITRendezvousVtbl {
    pub const fn new<Impl: ITRendezvousImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITRendezvousVtbl {
        unsafe extern "system" fn DefaultDirectories<Impl: ITRendezvousImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultDirectories(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDefaultDirectories<Impl: ITRendezvousImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdirectory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateDefaultDirectories(::core::mem::transmute_copy(&ppenumdirectory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectory<Impl: ITRendezvousImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, directorytype: DIRECTORY_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdir: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDirectory(directorytype, &*(&pname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdir)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectoryObject<Impl: ITRendezvousImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdirectoryobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDirectoryObject(directoryobjecttype, &*(&pname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdirectoryobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITRendezvous>, base.5, DefaultDirectories::<Impl, OFFSET>, EnumerateDefaultDirectories::<Impl, OFFSET>, CreateDirectory::<Impl, OFFSET>, CreateDirectoryObject::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITRequestImpl: Sized + IDispatchImpl {
    fn MakeCall();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITRequest {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITRequest";
}
#[cfg(feature = "Win32_System_Com")]
impl ITRequestVtbl {
    pub const fn new<Impl: ITRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITRequestVtbl {
        unsafe extern "system" fn MakeCall<Impl: ITRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pappname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcalledparty: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomment: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeCall(
                &*(&pdestaddress as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pappname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcalledparty as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcomment as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITRequest>, base.5, MakeCall::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITRequestEventImpl: Sized + IDispatchImpl {
    fn RegistrationInstance();
    fn RequestMode();
    fn DestAddress();
    fn AppName();
    fn CalledParty();
    fn Comment();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITRequestEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITRequestEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITRequestEventVtbl {
    pub const fn new<Impl: ITRequestEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITRequestEventVtbl {
        unsafe extern "system" fn RegistrationInstance<Impl: ITRequestEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plregistrationinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegistrationInstance(::core::mem::transmute_copy(&plregistrationinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMode<Impl: ITRequestEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plrequestmode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestMode(::core::mem::transmute_copy(&plrequestmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestAddress<Impl: ITRequestEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdestaddress: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DestAddress(::core::mem::transmute_copy(&ppdestaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppName<Impl: ITRequestEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppappname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppName(::core::mem::transmute_copy(&ppappname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalledParty<Impl: ITRequestEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcalledparty: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CalledParty(::core::mem::transmute_copy(&ppcalledparty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Impl: ITRequestEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcomment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Comment(::core::mem::transmute_copy(&ppcomment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITRequestEvent>, base.5, RegistrationInstance::<Impl, OFFSET>, RequestMode::<Impl, OFFSET>, DestAddress::<Impl, OFFSET>, AppName::<Impl, OFFSET>, CalledParty::<Impl, OFFSET>, Comment::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITScriptableAudioFormatImpl: Sized + IDispatchImpl {
    fn Channels();
    fn SetChannels();
    fn SamplesPerSec();
    fn SetSamplesPerSec();
    fn AvgBytesPerSec();
    fn SetAvgBytesPerSec();
    fn BlockAlign();
    fn SetBlockAlign();
    fn BitsPerSample();
    fn SetBitsPerSample();
    fn FormatTag();
    fn SetFormatTag();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITScriptableAudioFormat {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITScriptableAudioFormat";
}
#[cfg(feature = "Win32_System_Com")]
impl ITScriptableAudioFormatVtbl {
    pub const fn new<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITScriptableAudioFormatVtbl {
        unsafe extern "system" fn Channels<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Channels(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetChannels(nnewval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SamplesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SamplesPerSec(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSamplesPerSec(nnewval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvgBytesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AvgBytesPerSec(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAvgBytesPerSec(nnewval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockAlign<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BlockAlign(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBlockAlign(nnewval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitsPerSample<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BitsPerSample(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBitsPerSample(nnewval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatTag<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormatTag(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Impl: ITScriptableAudioFormatImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFormatTag(nnewval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITScriptableAudioFormat>, base.5, Channels::<Impl, OFFSET>, SetChannels::<Impl, OFFSET>, SamplesPerSec::<Impl, OFFSET>, SetSamplesPerSec::<Impl, OFFSET>, AvgBytesPerSec::<Impl, OFFSET>, SetAvgBytesPerSec::<Impl, OFFSET>, BlockAlign::<Impl, OFFSET>, SetBlockAlign::<Impl, OFFSET>, BitsPerSample::<Impl, OFFSET>, SetBitsPerSample::<Impl, OFFSET>, FormatTag::<Impl, OFFSET>, SetFormatTag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITStaticAudioTerminalImpl: Sized + IDispatchImpl {
    fn WaveId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITStaticAudioTerminal {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITStaticAudioTerminal";
}
#[cfg(feature = "Win32_System_Com")]
impl ITStaticAudioTerminalVtbl {
    pub const fn new<Impl: ITStaticAudioTerminalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITStaticAudioTerminalVtbl {
        unsafe extern "system" fn WaveId<Impl: ITStaticAudioTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwaveid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaveId(::core::mem::transmute_copy(&plwaveid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITStaticAudioTerminal>, base.5, WaveId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITStreamImpl: Sized + IDispatchImpl {
    fn MediaType();
    fn Direction();
    fn Name();
    fn StartStream();
    fn PauseStream();
    fn StopStream();
    fn SelectTerminal();
    fn UnselectTerminal();
    fn EnumerateTerminals();
    fn Terminals();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITStream {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITStream";
}
#[cfg(feature = "Win32_System_Com")]
impl ITStreamVtbl {
    pub const fn new<Impl: ITStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITStreamVtbl {
        unsafe extern "system" fn MediaType<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaType(::core::mem::transmute_copy(&plmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptd: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direction(::core::mem::transmute_copy(&ptd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartStream<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseStream<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PauseStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopStream<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectTerminal<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectTerminal(&*(&pterminal as *const <ITTerminal as ::windows::core::Abi>::Abi as *const <ITTerminal as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnselectTerminal<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnselectTerminal(&*(&pterminal as *const <ITTerminal as ::windows::core::Abi>::Abi as *const <ITTerminal as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateTerminals(::core::mem::transmute_copy(&ppenumterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Impl: ITStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminals(::core::mem::transmute_copy(&pterminals)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITStream>, base.5, MediaType::<Impl, OFFSET>, Direction::<Impl, OFFSET>, Name::<Impl, OFFSET>, StartStream::<Impl, OFFSET>, PauseStream::<Impl, OFFSET>, StopStream::<Impl, OFFSET>, SelectTerminal::<Impl, OFFSET>, UnselectTerminal::<Impl, OFFSET>, EnumerateTerminals::<Impl, OFFSET>, Terminals::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITStreamControlImpl: Sized + IDispatchImpl {
    fn CreateStream();
    fn RemoveStream();
    fn EnumerateStreams();
    fn Streams();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITStreamControl {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITStreamControl";
}
#[cfg(feature = "Win32_System_Com")]
impl ITStreamControlVtbl {
    pub const fn new<Impl: ITStreamControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITStreamControlVtbl {
        unsafe extern "system" fn CreateStream<Impl: ITStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateStream(lmediatype, td, ::core::mem::transmute_copy(&ppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Impl: ITStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveStream(&*(&pstream as *const <ITStream as ::windows::core::Abi>::Abi as *const <ITStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateStreams<Impl: ITStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateStreams(::core::mem::transmute_copy(&ppenumstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Streams<Impl: ITStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Streams(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITStreamControl>, base.5, CreateStream::<Impl, OFFSET>, RemoveStream::<Impl, OFFSET>, EnumerateStreams::<Impl, OFFSET>, Streams::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITSubStreamImpl: Sized + IDispatchImpl {
    fn StartSubStream();
    fn PauseSubStream();
    fn StopSubStream();
    fn SelectTerminal();
    fn UnselectTerminal();
    fn EnumerateTerminals();
    fn Terminals();
    fn Stream();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITSubStream {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITSubStream";
}
#[cfg(feature = "Win32_System_Com")]
impl ITSubStreamVtbl {
    pub const fn new<Impl: ITSubStreamImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITSubStreamVtbl {
        unsafe extern "system" fn StartSubStream<Impl: ITSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartSubStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseSubStream<Impl: ITSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PauseSubStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopSubStream<Impl: ITSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopSubStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectTerminal<Impl: ITSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectTerminal(&*(&pterminal as *const <ITTerminal as ::windows::core::Abi>::Abi as *const <ITTerminal as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnselectTerminal<Impl: ITSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnselectTerminal(&*(&pterminal as *const <ITTerminal as ::windows::core::Abi>::Abi as *const <ITTerminal as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTerminals<Impl: ITSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateTerminals(::core::mem::transmute_copy(&ppenumterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminals<Impl: ITSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminals(::core::mem::transmute_copy(&pterminals)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: ITSubStreamImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stream(::core::mem::transmute_copy(&ppitstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITSubStream>, base.5, StartSubStream::<Impl, OFFSET>, PauseSubStream::<Impl, OFFSET>, StopSubStream::<Impl, OFFSET>, SelectTerminal::<Impl, OFFSET>, UnselectTerminal::<Impl, OFFSET>, EnumerateTerminals::<Impl, OFFSET>, Terminals::<Impl, OFFSET>, Stream::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITSubStreamControlImpl: Sized + IDispatchImpl {
    fn CreateSubStream();
    fn RemoveSubStream();
    fn EnumerateSubStreams();
    fn SubStreams();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITSubStreamControl {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITSubStreamControl";
}
#[cfg(feature = "Win32_System_Com")]
impl ITSubStreamControlVtbl {
    pub const fn new<Impl: ITSubStreamControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITSubStreamControlVtbl {
        unsafe extern "system" fn CreateSubStream<Impl: ITSubStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSubStream(::core::mem::transmute_copy(&ppsubstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubStream<Impl: ITSubStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveSubStream(&*(&psubstream as *const <ITSubStream as ::windows::core::Abi>::Abi as *const <ITSubStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateSubStreams<Impl: ITSubStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumsubstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateSubStreams(::core::mem::transmute_copy(&ppenumsubstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubStreams<Impl: ITSubStreamControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SubStreams(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITSubStreamControl>, base.5, CreateSubStream::<Impl, OFFSET>, RemoveSubStream::<Impl, OFFSET>, EnumerateSubStreams::<Impl, OFFSET>, SubStreams::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Shutdown();
    fn Addresses();
    fn EnumerateAddresses();
    fn RegisterCallNotifications();
    fn UnregisterNotifications();
    fn CallHubs();
    fn EnumerateCallHubs();
    fn SetCallHubTracking();
    fn EnumeratePrivateTAPIObjects();
    fn PrivateTAPIObjects();
    fn RegisterRequestRecipient();
    fn SetAssistedTelephonyPriority();
    fn SetApplicationPriority();
    fn SetEventFilter();
    fn EventFilter();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTAPI {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTAPI";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIVtbl {
    pub const fn new<Impl: ITTAPIImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTAPIVtbl {
        unsafe extern "system" fn Initialize<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Addresses<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Addresses(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAddresses<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateAddresses(::core::mem::transmute_copy(&ppenumaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCallNotifications<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr, fmonitor: i16, fowner: i16, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterCallNotifications(&*(&paddress as *const <ITAddress as ::windows::core::Abi>::Abi as *const <ITAddress as ::windows::core::DefaultType>::DefaultType), fmonitor, fowner, lmediatypes, lcallbackinstance, ::core::mem::transmute_copy(&plregister)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterNotifications<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lregister: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterNotifications(lregister) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallHubs<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallHubs(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateCallHubs<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcallhub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateCallHubs(::core::mem::transmute_copy(&ppenumcallhub)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCallHubTracking<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddresses: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, btracking: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCallHubTracking(&*(&paddresses as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), btracking) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePrivateTAPIObjects<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumeratePrivateTAPIObjects(::core::mem::transmute_copy(&ppenumunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateTAPIObjects<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateTAPIObjects(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterRequestRecipient<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lregistrationinstance: i32, lrequestmode: i32, fenable: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterRequestRecipient(lregistrationinstance, lrequestmode, fenable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAssistedTelephonyPriority<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAssistedTelephonyPriority(&*(&pappfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), fpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationPriority<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, fpriority: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetApplicationPriority(&*(&pappfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lmediatype, fpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventFilter<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfiltermask: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEventFilter(lfiltermask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventFilter<Impl: ITTAPIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plfiltermask: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EventFilter(::core::mem::transmute_copy(&plfiltermask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ITTAPI>,
            base.5,
            Initialize::<Impl, OFFSET>,
            Shutdown::<Impl, OFFSET>,
            Addresses::<Impl, OFFSET>,
            EnumerateAddresses::<Impl, OFFSET>,
            RegisterCallNotifications::<Impl, OFFSET>,
            UnregisterNotifications::<Impl, OFFSET>,
            CallHubs::<Impl, OFFSET>,
            EnumerateCallHubs::<Impl, OFFSET>,
            SetCallHubTracking::<Impl, OFFSET>,
            EnumeratePrivateTAPIObjects::<Impl, OFFSET>,
            PrivateTAPIObjects::<Impl, OFFSET>,
            RegisterRequestRecipient::<Impl, OFFSET>,
            SetAssistedTelephonyPriority::<Impl, OFFSET>,
            SetApplicationPriority::<Impl, OFFSET>,
            SetEventFilter::<Impl, OFFSET>,
            EventFilter::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPI2Impl: Sized + ITTAPIImpl + IDispatchImpl {
    fn Phones();
    fn EnumeratePhones();
    fn CreateEmptyCollectionObject();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTAPI2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTAPI2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPI2Vtbl {
    pub const fn new<Impl: ITTAPI2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTAPI2Vtbl {
        unsafe extern "system" fn Phones<Impl: ITTAPI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Phones(::core::mem::transmute_copy(&pphones)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePhones<Impl: ITTAPI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumeratePhones(::core::mem::transmute_copy(&ppenumphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEmptyCollectionObject<Impl: ITTAPI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateEmptyCollectionObject(::core::mem::transmute_copy(&ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTAPI2>, base.5, Phones::<Impl, OFFSET>, EnumeratePhones::<Impl, OFFSET>, CreateEmptyCollectionObject::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPICallCenterImpl: Sized + IDispatchImpl {
    fn EnumerateAgentHandlers();
    fn AgentHandlers();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTAPICallCenter {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTAPICallCenter";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPICallCenterVtbl {
    pub const fn new<Impl: ITTAPICallCenterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTAPICallCenterVtbl {
        unsafe extern "system" fn EnumerateAgentHandlers<Impl: ITTAPICallCenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumhandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateAgentHandlers(::core::mem::transmute_copy(&ppenumhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AgentHandlers<Impl: ITTAPICallCenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AgentHandlers(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTAPICallCenter>, base.5, EnumerateAgentHandlers::<Impl, OFFSET>, AgentHandlers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIDispatchEventNotificationImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTAPIDispatchEventNotification {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTAPIDispatchEventNotification";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIDispatchEventNotificationVtbl {
    pub const fn new<Impl: ITTAPIDispatchEventNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTAPIDispatchEventNotificationVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTAPIDispatchEventNotification>, base.5)
    }
}
pub trait ITTAPIEventNotificationImpl: Sized {
    fn Event();
}
impl ::windows::core::RuntimeName for ITTAPIEventNotification {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTAPIEventNotification";
}
impl ITTAPIEventNotificationVtbl {
    pub const fn new<Impl: ITTAPIEventNotificationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTAPIEventNotificationVtbl {
        unsafe extern "system" fn Event<Impl: ITTAPIEventNotificationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(tapievent, &*(&pevent as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTAPIEventNotification>, base.5, Event::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIObjectEventImpl: Sized + IDispatchImpl {
    fn TAPIObject();
    fn Event();
    fn Address();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTAPIObjectEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTAPIObjectEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIObjectEventVtbl {
    pub const fn new<Impl: ITTAPIObjectEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTAPIObjectEventVtbl {
        unsafe extern "system" fn TAPIObject<Impl: ITTAPIObjectEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TAPIObject(::core::mem::transmute_copy(&pptapiobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Event<Impl: ITTAPIObjectEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut TAPIOBJECT_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Event(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: ITTAPIObjectEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address(::core::mem::transmute_copy(&ppaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITTAPIObjectEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallbackInstance(::core::mem::transmute_copy(&plcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTAPIObjectEvent>, base.5, TAPIObject::<Impl, OFFSET>, Event::<Impl, OFFSET>, Address::<Impl, OFFSET>, CallbackInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIObjectEvent2Impl: Sized + ITTAPIObjectEventImpl + IDispatchImpl {
    fn Phone();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTAPIObjectEvent2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTAPIObjectEvent2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIObjectEvent2Vtbl {
    pub const fn new<Impl: ITTAPIObjectEvent2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTAPIObjectEvent2Vtbl {
        unsafe extern "system" fn Phone<Impl: ITTAPIObjectEvent2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppphone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Phone(::core::mem::transmute_copy(&ppphone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTAPIObjectEvent2>, base.5, Phone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTTSTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Call();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTTSTerminalEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTTSTerminalEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTTSTerminalEventVtbl {
    pub const fn new<Impl: ITTTSTerminalEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTTSTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITTTSTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminal(::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITTTSTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITTTSTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error(::core::mem::transmute_copy(&phrerrorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTTSTerminalEvent>, base.5, Terminal::<Impl, OFFSET>, Call::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTerminalImpl: Sized + IDispatchImpl {
    fn Name();
    fn State();
    fn TerminalType();
    fn TerminalClass();
    fn MediaType();
    fn Direction();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTerminal {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTerminal";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalVtbl {
    pub const fn new<Impl: ITTerminalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTerminalVtbl {
        unsafe extern "system" fn Name<Impl: ITTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ITTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminalstate: *mut TERMINAL_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State(::core::mem::transmute_copy(&pterminalstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalType<Impl: ITTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut TERMINAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TerminalType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminalClass<Impl: ITTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminalclass: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TerminalClass(::core::mem::transmute_copy(&ppterminalclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaType<Impl: ITTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MediaType(::core::mem::transmute_copy(&plmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ITTerminalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direction(::core::mem::transmute_copy(&pdirection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTerminal>, base.5, Name::<Impl, OFFSET>, State::<Impl, OFFSET>, TerminalType::<Impl, OFFSET>, TerminalClass::<Impl, OFFSET>, MediaType::<Impl, OFFSET>, Direction::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTerminalSupportImpl: Sized + IDispatchImpl {
    fn StaticTerminals();
    fn EnumerateStaticTerminals();
    fn DynamicTerminalClasses();
    fn EnumerateDynamicTerminalClasses();
    fn CreateTerminal();
    fn GetDefaultStaticTerminal();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTerminalSupport {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTerminalSupport";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalSupportVtbl {
    pub const fn new<Impl: ITTerminalSupportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTerminalSupportVtbl {
        unsafe extern "system" fn StaticTerminals<Impl: ITTerminalSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StaticTerminals(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateStaticTerminals<Impl: ITTerminalSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminalenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateStaticTerminals(::core::mem::transmute_copy(&ppterminalenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DynamicTerminalClasses<Impl: ITTerminalSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DynamicTerminalClasses(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateDynamicTerminalClasses<Impl: ITTerminalSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminalclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerateDynamicTerminalClasses(::core::mem::transmute_copy(&ppterminalclassenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTerminal<Impl: ITTerminalSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminalclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTerminal(&*(&pterminalclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lmediatype, direction, ::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultStaticTerminal<Impl: ITTerminalSupportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultStaticTerminal(lmediatype, direction, ::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTerminalSupport>, base.5, StaticTerminals::<Impl, OFFSET>, EnumerateStaticTerminals::<Impl, OFFSET>, DynamicTerminalClasses::<Impl, OFFSET>, EnumerateDynamicTerminalClasses::<Impl, OFFSET>, CreateTerminal::<Impl, OFFSET>, GetDefaultStaticTerminal::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITTerminalSupport2Impl: Sized + ITTerminalSupportImpl + IDispatchImpl {
    fn PluggableSuperclasses();
    fn EnumeratePluggableSuperclasses();
    fn PluggableTerminalClasses();
    fn EnumeratePluggableTerminalClasses();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITTerminalSupport2 {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITTerminalSupport2";
}
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalSupport2Vtbl {
    pub const fn new<Impl: ITTerminalSupport2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITTerminalSupport2Vtbl {
        unsafe extern "system" fn PluggableSuperclasses<Impl: ITTerminalSupport2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PluggableSuperclasses(::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableSuperclasses<Impl: ITTerminalSupport2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsuperclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumeratePluggableSuperclasses(::core::mem::transmute_copy(&ppsuperclassenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PluggableTerminalClasses<Impl: ITTerminalSupport2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrterminalsuperclass: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmediatype: i32, pvariant: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PluggableTerminalClasses(&*(&bstrterminalsuperclass as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lmediatype, ::core::mem::transmute_copy(&pvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumeratePluggableTerminalClasses<Impl: ITTerminalSupport2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iidterminalsuperclass: ::windows::core::GUID, lmediatype: i32, ppclassenumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumeratePluggableTerminalClasses(&*(&iidterminalsuperclass as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lmediatype, ::core::mem::transmute_copy(&ppclassenumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITTerminalSupport2>, base.5, PluggableSuperclasses::<Impl, OFFSET>, EnumeratePluggableSuperclasses::<Impl, OFFSET>, PluggableTerminalClasses::<Impl, OFFSET>, EnumeratePluggableTerminalClasses::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITToneDetectionEventImpl: Sized + IDispatchImpl {
    fn Call();
    fn AppSpecific();
    fn TickCount();
    fn CallbackInstance();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITToneDetectionEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITToneDetectionEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITToneDetectionEventVtbl {
    pub const fn new<Impl: ITToneDetectionEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITToneDetectionEventVtbl {
        unsafe extern "system" fn Call<Impl: ITToneDetectionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcallinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppSpecific<Impl: ITToneDetectionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppSpecific(::core::mem::transmute_copy(&plappspecific)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Impl: ITToneDetectionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TickCount(::core::mem::transmute_copy(&pltickcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallbackInstance<Impl: ITToneDetectionEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CallbackInstance(::core::mem::transmute_copy(&plcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITToneDetectionEvent>, base.5, Call::<Impl, OFFSET>, AppSpecific::<Impl, OFFSET>, TickCount::<Impl, OFFSET>, CallbackInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITToneTerminalEventImpl: Sized + IDispatchImpl {
    fn Terminal();
    fn Call();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITToneTerminalEvent {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITToneTerminalEvent";
}
#[cfg(feature = "Win32_System_Com")]
impl ITToneTerminalEventVtbl {
    pub const fn new<Impl: ITToneTerminalEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITToneTerminalEventVtbl {
        unsafe extern "system" fn Terminal<Impl: ITToneTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Terminal(::core::mem::transmute_copy(&ppterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Call<Impl: ITToneTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Call(::core::mem::transmute_copy(&ppcall)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: ITToneTerminalEventImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error(::core::mem::transmute_copy(&phrerrorcode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITToneTerminalEvent>, base.5, Terminal::<Impl, OFFSET>, Call::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
pub trait ITnefImpl: Sized {
    fn AddProps();
    fn ExtractProps();
    fn Finish();
    fn OpenTaggedBody();
    fn SetProps();
    fn EncodeRecips();
    fn FinishComponent();
}
impl ::windows::core::RuntimeName for ITnef {
    const NAME: &'static str = "Windows.Win32.Devices.Tapi.ITnef";
}
impl ITnefVtbl {
    pub const fn new<Impl: ITnefImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITnefVtbl {
        unsafe extern "system" fn AddProps<Impl: ITnefImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddProps(ulflags, ulelemid, &*(&lpvdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&lpproplist as *const <super::super::System::AddressBook::SPropTagArray as ::windows::core::Abi>::Abi as *const <super::super::System::AddressBook::SPropTagArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtractProps<Impl: ITnefImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtractProps(ulflags, &*(&lpproplist as *const <super::super::System::AddressBook::SPropTagArray as ::windows::core::Abi>::Abi as *const <super::super::System::AddressBook::SPropTagArray as ::windows::core::DefaultType>::DefaultType), &*(&lpproblems as *const <STnefProblemArray as ::windows::core::Abi>::Abi as *const <STnefProblemArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Finish<Impl: ITnefImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Finish(ulflags, lpkey, &*(&lpproblems as *const <STnefProblemArray as ::windows::core::Abi>::Abi as *const <STnefProblemArray as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTaggedBody<Impl: ITnefImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpmessage: ::windows::core::RawPtr, ulflags: u32, lppstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenTaggedBody(&*(&lpmessage as *const <super::super::System::AddressBook::IMessage as ::windows::core::Abi>::Abi as *const <super::super::System::AddressBook::IMessage as ::windows::core::DefaultType>::DefaultType), ulflags, ::core::mem::transmute_copy(&lppstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProps<Impl: ITnefImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProps(ulflags, ulelemid, cvalues, &*(&lpprops as *const <super::super::System::AddressBook::SPropValue as ::windows::core::Abi>::Abi as *const <super::super::System::AddressBook::SPropValue as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodeRecips<Impl: ITnefImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lprecipienttable: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EncodeRecips(ulflags, &*(&lprecipienttable as *const <super::super::System::AddressBook::IMAPITable as ::windows::core::Abi>::Abi as *const <super::super::System::AddressBook::IMAPITable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishComponent<Impl: ITnefImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FinishComponent(
                ulflags,
                ulcomponentid,
                &*(&lpcustomproplist as *const <super::super::System::AddressBook::SPropTagArray as ::windows::core::Abi>::Abi as *const <super::super::System::AddressBook::SPropTagArray as ::windows::core::DefaultType>::DefaultType),
                &*(&lpcustomprops as *const <super::super::System::AddressBook::SPropValue as ::windows::core::Abi>::Abi as *const <super::super::System::AddressBook::SPropValue as ::windows::core::DefaultType>::DefaultType),
                &*(&lpproplist as *const <super::super::System::AddressBook::SPropTagArray as ::windows::core::Abi>::Abi as *const <super::super::System::AddressBook::SPropTagArray as ::windows::core::DefaultType>::DefaultType),
                &*(&lpproblems as *const <STnefProblemArray as ::windows::core::Abi>::Abi as *const <STnefProblemArray as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITnef>, base.5, AddProps::<Impl, OFFSET>, ExtractProps::<Impl, OFFSET>, Finish::<Impl, OFFSET>, OpenTaggedBody::<Impl, OFFSET>, SetProps::<Impl, OFFSET>, EncodeRecips::<Impl, OFFSET>, FinishComponent::<Impl, OFFSET>)
    }
}
