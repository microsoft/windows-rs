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
    pub const fn new<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPwmControllerProviderVtbl {
        unsafe extern "system" fn PinCount<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PinCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualFrequency<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredFrequency<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frequency: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDesiredFrequency(frequency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxFrequency<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinFrequency<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquirePin<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AcquirePin(pin).into()
        }
        unsafe extern "system" fn ReleasePin<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReleasePin(pin).into()
        }
        unsafe extern "system" fn EnablePin<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnablePin(pin).into()
        }
        unsafe extern "system" fn DisablePin<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DisablePin(pin).into()
        }
        unsafe extern "system" fn SetPulseParameters<Impl: IPwmControllerProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPulseParameters(pin, dutycycle, invertpolarity).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPwmControllerProvider>, base.5, PinCount::<Impl, OFFSET>, ActualFrequency::<Impl, OFFSET>, SetDesiredFrequency::<Impl, OFFSET>, MaxFrequency::<Impl, OFFSET>, MinFrequency::<Impl, OFFSET>, AcquirePin::<Impl, OFFSET>, ReleasePin::<Impl, OFFSET>, EnablePin::<Impl, OFFSET>, DisablePin::<Impl, OFFSET>, SetPulseParameters::<Impl, OFFSET>)
    }
}
pub trait IPwmProviderImpl: Sized {
    fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>>;
}
impl ::windows::core::RuntimeName for IPwmProvider {
    const NAME: &'static str = "Windows.Devices.Pwm.Provider.IPwmProvider";
}
impl IPwmProviderVtbl {
    pub const fn new<Impl: IPwmProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPwmProviderVtbl {
        unsafe extern "system" fn GetControllers<Impl: IPwmProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPwmProvider>, base.5, GetControllers::<Impl, OFFSET>)
    }
}
