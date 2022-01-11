#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
pub trait ILowLevelDevicesAggregateProviderImpl: Sized {
    fn AdcControllerProvider(&self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider>;
    fn PwmControllerProvider(&self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider>;
    fn GpioControllerProvider(&self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider>;
    fn I2cControllerProvider(&self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider>;
    fn SpiControllerProvider(&self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider>;
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl ::windows::core::RuntimeName for ILowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.ILowLevelDevicesAggregateProvider";
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl ILowLevelDevicesAggregateProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLevelDevicesAggregateProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLevelDevicesAggregateProviderVtbl {
        unsafe extern "system" fn AdcControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdcControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PwmControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PwmControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GpioControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GpioControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn I2cControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).I2cControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpiControllerProvider<Impl: ILowLevelDevicesAggregateProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpiControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLevelDevicesAggregateProvider, BASE_OFFSET>(),
            AdcControllerProvider: AdcControllerProvider::<Impl, IMPL_OFFSET>,
            PwmControllerProvider: PwmControllerProvider::<Impl, IMPL_OFFSET>,
            GpioControllerProvider: GpioControllerProvider::<Impl, IMPL_OFFSET>,
            I2cControllerProvider: I2cControllerProvider::<Impl, IMPL_OFFSET>,
            SpiControllerProvider: SpiControllerProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLevelDevicesAggregateProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider", feature = "implement_exclusive"))]
pub trait ILowLevelDevicesAggregateProviderFactoryImpl: Sized {
    fn Create(&self, adc: &::core::option::Option<Adc::Provider::IAdcControllerProvider>, pwm: &::core::option::Option<Pwm::Provider::IPwmControllerProvider>, gpio: &::core::option::Option<Gpio::Provider::IGpioControllerProvider>, i2c: &::core::option::Option<I2c::Provider::II2cControllerProvider>, spi: &::core::option::Option<Spi::Provider::ISpiControllerProvider>) -> ::windows::core::Result<LowLevelDevicesAggregateProvider>;
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILowLevelDevicesAggregateProviderFactory {
    const NAME: &'static str = "Windows.Devices.ILowLevelDevicesAggregateProviderFactory";
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider", feature = "implement_exclusive"))]
impl ILowLevelDevicesAggregateProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLevelDevicesAggregateProviderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLevelDevicesAggregateProviderFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ILowLevelDevicesAggregateProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adc: ::windows::core::RawPtr, pwm: ::windows::core::RawPtr, gpio: ::windows::core::RawPtr, i2c: ::windows::core::RawPtr, spi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLevelDevicesAggregateProviderFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLevelDevicesAggregateProviderFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLevelDevicesControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLevelDevicesControllerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLevelDevicesController, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLevelDevicesController as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLevelDevicesControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLevelDevicesControllerStaticsVtbl {
        unsafe extern "system" fn DefaultProvider<Impl: ILowLevelDevicesControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultProvider<Impl: ILowLevelDevicesControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultProvider(&*(&value as *const <ILowLevelDevicesAggregateProvider as ::windows::core::Abi>::Abi as *const <ILowLevelDevicesAggregateProvider as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILowLevelDevicesControllerStatics, BASE_OFFSET>(),
            DefaultProvider: DefaultProvider::<Impl, IMPL_OFFSET>,
            SetDefaultProvider: SetDefaultProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILowLevelDevicesControllerStatics as ::windows::core::Interface>::IID
    }
}
