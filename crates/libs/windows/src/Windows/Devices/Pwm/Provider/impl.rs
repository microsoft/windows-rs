pub trait IPwmControllerProviderImpl: Sized {
    fn PinCount(&self) -> ::windows::core::Result<i32>;
    fn ActualFrequency(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredFrequency(&self, frequency: f64) -> ::windows::core::Result<f64>;
    fn MaxFrequency(&self) -> ::windows::core::Result<f64>;
    fn MinFrequency(&self) -> ::windows::core::Result<f64>;
    fn AcquirePin(&self, pin: i32) -> ::windows::core::Result<()>;
    fn ReleasePin(&self, pin: i32) -> ::windows::core::Result<()>;
    fn EnablePin(&self, pin: i32) -> ::windows::core::Result<()>;
    fn DisablePin(&self, pin: i32) -> ::windows::core::Result<()>;
    fn SetPulseParameters(&self, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPwmControllerProvider {
    const NAME: &'static str = "Windows.Devices.Pwm.Provider.IPwmControllerProvider";
}
impl IPwmControllerProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPwmControllerProviderVtbl {
        unsafe extern "system" fn PinCount<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualFrequency<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredFrequency<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredFrequency(frequency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxFrequency<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinFrequency<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquirePin<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquirePin(pin).into()
        }
        unsafe extern "system" fn ReleasePin<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleasePin(pin).into()
        }
        unsafe extern "system" fn EnablePin<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnablePin(pin).into()
        }
        unsafe extern "system" fn DisablePin<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisablePin(pin).into()
        }
        unsafe extern "system" fn SetPulseParameters<Impl: IPwmControllerProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPulseParameters(pin, dutycycle, invertpolarity).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPwmControllerProvider>,
            ::windows::core::GetTrustLevel,
            PinCount::<Impl, IMPL_OFFSET>,
            ActualFrequency::<Impl, IMPL_OFFSET>,
            SetDesiredFrequency::<Impl, IMPL_OFFSET>,
            MaxFrequency::<Impl, IMPL_OFFSET>,
            MinFrequency::<Impl, IMPL_OFFSET>,
            AcquirePin::<Impl, IMPL_OFFSET>,
            ReleasePin::<Impl, IMPL_OFFSET>,
            EnablePin::<Impl, IMPL_OFFSET>,
            DisablePin::<Impl, IMPL_OFFSET>,
            SetPulseParameters::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPwmControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPwmProviderImpl: Sized {
    fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPwmProvider {
    const NAME: &'static str = "Windows.Devices.Pwm.Provider.IPwmProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl IPwmProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPwmProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPwmProviderVtbl {
        unsafe extern "system" fn GetControllers<Impl: IPwmProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPwmProvider>, ::windows::core::GetTrustLevel, GetControllers::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPwmProvider as ::windows::core::Interface>::IID
    }
}
