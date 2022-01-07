pub trait ILowLevelDevicesAggregateProviderImpl: Sized {
    fn AdcControllerProvider(&self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider>;
    fn PwmControllerProvider(&self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider>;
    fn GpioControllerProvider(&self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider>;
    fn I2cControllerProvider(&self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider>;
    fn SpiControllerProvider(&self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider>;
}
impl ::windows::core::RuntimeName for ILowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.ILowLevelDevicesAggregateProvider";
}
impl ILowLevelDevicesAggregateProviderVtbl {
    pub const fn new<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLevelDevicesAggregateProviderVtbl {
        unsafe extern "system" fn AdcControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdcControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PwmControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PwmControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GpioControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GpioControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn I2cControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).I2cControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpiControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpiControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLevelDevicesAggregateProvider>, base.5, AdcControllerProvider::<Impl, OFFSET>, PwmControllerProvider::<Impl, OFFSET>, GpioControllerProvider::<Impl, OFFSET>, I2cControllerProvider::<Impl, OFFSET>, SpiControllerProvider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLevelDevicesAggregateProviderFactoryImpl: Sized {
    fn Create(&self, adc: &::core::option::Option<Adc::Provider::IAdcControllerProvider>, pwm: &::core::option::Option<Pwm::Provider::IPwmControllerProvider>, gpio: &::core::option::Option<Gpio::Provider::IGpioControllerProvider>, i2c: &::core::option::Option<I2c::Provider::II2cControllerProvider>, spi: &::core::option::Option<Spi::Provider::ISpiControllerProvider>) -> ::windows::core::Result<LowLevelDevicesAggregateProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLevelDevicesAggregateProviderFactory {
    const NAME: &'static str = "Windows.Devices.ILowLevelDevicesAggregateProviderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLevelDevicesAggregateProviderFactoryVtbl {
    pub const fn new<Impl: ILowLevelDevicesAggregateProviderFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLevelDevicesAggregateProviderFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ILowLevelDevicesAggregateProviderFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adc: ::windows::core::RawPtr, pwm: ::windows::core::RawPtr, gpio: ::windows::core::RawPtr, i2c: ::windows::core::RawPtr, spi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&adc as *const <Adc::Provider::IAdcControllerProvider as ::windows::core::Abi>::Abi as *const <Adc::Provider::IAdcControllerProvider as ::windows::core::DefaultType>::DefaultType),
                &*(&pwm as *const <Pwm::Provider::IPwmControllerProvider as ::windows::core::Abi>::Abi as *const <Pwm::Provider::IPwmControllerProvider as ::windows::core::DefaultType>::DefaultType),
                &*(&gpio as *const <Gpio::Provider::IGpioControllerProvider as ::windows::core::Abi>::Abi as *const <Gpio::Provider::IGpioControllerProvider as ::windows::core::DefaultType>::DefaultType),
                &*(&i2c as *const <I2c::Provider::II2cControllerProvider as ::windows::core::Abi>::Abi as *const <I2c::Provider::II2cControllerProvider as ::windows::core::DefaultType>::DefaultType),
                &*(&spi as *const <Spi::Provider::ISpiControllerProvider as ::windows::core::Abi>::Abi as *const <Spi::Provider::ISpiControllerProvider as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLevelDevicesAggregateProviderFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLevelDevicesControllerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLevelDevicesController {
    const NAME: &'static str = "Windows.Devices.ILowLevelDevicesController";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLevelDevicesControllerVtbl {
    pub const fn new<Impl: ILowLevelDevicesControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLevelDevicesControllerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLevelDevicesController>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILowLevelDevicesControllerStaticsImpl: Sized {
    fn DefaultProvider(&self) -> ::windows::core::Result<ILowLevelDevicesAggregateProvider>;
    fn SetDefaultProvider(&self, value: &::core::option::Option<ILowLevelDevicesAggregateProvider>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILowLevelDevicesControllerStatics {
    const NAME: &'static str = "Windows.Devices.ILowLevelDevicesControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILowLevelDevicesControllerStaticsVtbl {
    pub const fn new<Impl: ILowLevelDevicesControllerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILowLevelDevicesControllerStaticsVtbl {
        unsafe extern "system" fn DefaultProvider<Impl: ILowLevelDevicesControllerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultProvider<Impl: ILowLevelDevicesControllerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultProvider(&*(&value as *const <ILowLevelDevicesAggregateProvider as ::windows::core::Abi>::Abi as *const <ILowLevelDevicesAggregateProvider as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILowLevelDevicesControllerStatics>, base.5, DefaultProvider::<Impl, OFFSET>, SetDefaultProvider::<Impl, OFFSET>)
    }
}
