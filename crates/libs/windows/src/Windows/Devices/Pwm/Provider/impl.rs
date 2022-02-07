pub trait IPwmControllerProvider_Impl: Sized {
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
impl IPwmControllerProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>() -> IPwmControllerProvider_Vtbl {
        unsafe extern "system" fn PinCount<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PinCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualFrequency<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ActualFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredFrequency<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frequency: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetDesiredFrequency(frequency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxFrequency<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinFrequency<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MinFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquirePin<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquirePin(pin).into()
        }
        unsafe extern "system" fn ReleasePin<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleasePin(pin).into()
        }
        unsafe extern "system" fn EnablePin<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnablePin(pin).into()
        }
        unsafe extern "system" fn DisablePin<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisablePin(pin).into()
        }
        unsafe extern "system" fn SetPulseParameters<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPulseParameters(pin, dutycycle, invertpolarity).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPwmControllerProvider, OFFSET>(),
            PinCount: PinCount::<Identity, Impl, OFFSET>,
            ActualFrequency: ActualFrequency::<Identity, Impl, OFFSET>,
            SetDesiredFrequency: SetDesiredFrequency::<Identity, Impl, OFFSET>,
            MaxFrequency: MaxFrequency::<Identity, Impl, OFFSET>,
            MinFrequency: MinFrequency::<Identity, Impl, OFFSET>,
            AcquirePin: AcquirePin::<Identity, Impl, OFFSET>,
            ReleasePin: ReleasePin::<Identity, Impl, OFFSET>,
            EnablePin: EnablePin::<Identity, Impl, OFFSET>,
            DisablePin: DisablePin::<Identity, Impl, OFFSET>,
            SetPulseParameters: SetPulseParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPwmControllerProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPwmProvider_Impl: Sized {
    fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IPwmProvider {
    const NAME: &'static str = "Windows.Devices.Pwm.Provider.IPwmProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl IPwmProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPwmProvider_Impl, const OFFSET: isize>() -> IPwmProvider_Vtbl {
        unsafe extern "system" fn GetControllers<Identity: ::windows::core::IUnknownImpl, Impl: IPwmProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPwmProvider, OFFSET>(), GetControllers: GetControllers::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPwmProvider as ::windows::core::Interface>::IID
    }
}
