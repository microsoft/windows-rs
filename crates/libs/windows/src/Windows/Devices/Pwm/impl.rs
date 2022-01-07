#[cfg(feature = "implement_exclusive")]
pub trait IPwmControllerImpl: Sized {
    fn PinCount(&self) -> ::windows::core::Result<i32>;
    fn ActualFrequency(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredFrequency(&self, desiredfrequency: f64) -> ::windows::core::Result<f64>;
    fn MinFrequency(&self) -> ::windows::core::Result<f64>;
    fn MaxFrequency(&self) -> ::windows::core::Result<f64>;
    fn OpenPin(&self, pinnumber: i32) -> ::windows::core::Result<PwmPin>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPwmController {
    const NAME: &'static str = "Windows.Devices.Pwm.IPwmController";
}
#[cfg(feature = "implement_exclusive")]
impl IPwmControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerImpl, const OFFSET: isize>() -> IPwmControllerVtbl {
        unsafe extern "system" fn PinCount<Impl: IPwmControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActualFrequency<Impl: IPwmControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredFrequency<Impl: IPwmControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredfrequency: f64, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredFrequency(desiredfrequency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinFrequency<Impl: IPwmControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxFrequency<Impl: IPwmControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenPin<Impl: IPwmControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinnumber: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPin(pinnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPwmController>, ::windows::core::GetTrustLevel, PinCount::<Impl, OFFSET>, ActualFrequency::<Impl, OFFSET>, SetDesiredFrequency::<Impl, OFFSET>, MinFrequency::<Impl, OFFSET>, MaxFrequency::<Impl, OFFSET>, OpenPin::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPwmControllerStaticsImpl: Sized {
    fn GetControllersAsync(&self, provider: &::core::option::Option<Provider::IPwmProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PwmController>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPwmControllerStatics {
    const NAME: &'static str = "Windows.Devices.Pwm.IPwmControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPwmControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerStaticsImpl, const OFFSET: isize>() -> IPwmControllerStaticsVtbl {
        unsafe extern "system" fn GetControllersAsync<Impl: IPwmControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControllersAsync(&*(&provider as *const <Provider::IPwmProvider as ::windows::core::Abi>::Abi as *const <Provider::IPwmProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPwmControllerStatics>, ::windows::core::GetTrustLevel, GetControllersAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPwmControllerStatics2Impl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PwmController>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPwmControllerStatics2 {
    const NAME: &'static str = "Windows.Devices.Pwm.IPwmControllerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPwmControllerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerStatics2Impl, const OFFSET: isize>() -> IPwmControllerStatics2Vtbl {
        unsafe extern "system" fn GetDefaultAsync<Impl: IPwmControllerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPwmControllerStatics2>, ::windows::core::GetTrustLevel, GetDefaultAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPwmControllerStatics3Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PwmController>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPwmControllerStatics3 {
    const NAME: &'static str = "Windows.Devices.Pwm.IPwmControllerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IPwmControllerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPwmControllerStatics3Impl, const OFFSET: isize>() -> IPwmControllerStatics3Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IPwmControllerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromFriendlyName<Impl: IPwmControllerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromFriendlyName(&*(&friendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IPwmControllerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPwmControllerStatics3>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, OFFSET>, GetDeviceSelectorFromFriendlyName::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPwmPinImpl: Sized + IClosableImpl {
    fn Controller(&self) -> ::windows::core::Result<PwmController>;
    fn GetActiveDutyCyclePercentage(&self) -> ::windows::core::Result<f64>;
    fn SetActiveDutyCyclePercentage(&self, dutycyclepercentage: f64) -> ::windows::core::Result<()>;
    fn Polarity(&self) -> ::windows::core::Result<PwmPulsePolarity>;
    fn SetPolarity(&self, value: PwmPulsePolarity) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn IsStarted(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPwmPin {
    const NAME: &'static str = "Windows.Devices.Pwm.IPwmPin";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPwmPinVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPwmPinImpl, const OFFSET: isize>() -> IPwmPinVtbl {
        unsafe extern "system" fn Controller<Impl: IPwmPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDutyCyclePercentage<Impl: IPwmPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveDutyCyclePercentage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDutyCyclePercentage<Impl: IPwmPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dutycyclepercentage: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveDutyCyclePercentage(dutycyclepercentage).into()
        }
        unsafe extern "system" fn Polarity<Impl: IPwmPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PwmPulsePolarity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Polarity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPolarity<Impl: IPwmPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PwmPulsePolarity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPolarity(value).into()
        }
        unsafe extern "system" fn Start<Impl: IPwmPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPwmPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn IsStarted<Impl: IPwmPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStarted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPwmPin>,
            ::windows::core::GetTrustLevel,
            Controller::<Impl, OFFSET>,
            GetActiveDutyCyclePercentage::<Impl, OFFSET>,
            SetActiveDutyCyclePercentage::<Impl, OFFSET>,
            Polarity::<Impl, OFFSET>,
            SetPolarity::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            IsStarted::<Impl, OFFSET>,
        )
    }
}
