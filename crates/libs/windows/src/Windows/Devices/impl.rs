#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
pub trait ILowLevelDevicesAggregateProvider_Impl: Sized {
    fn AdcControllerProvider(&mut self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider>;
    fn PwmControllerProvider(&mut self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider>;
    fn GpioControllerProvider(&mut self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider>;
    fn I2cControllerProvider(&mut self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider>;
    fn SpiControllerProvider(&mut self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider>;
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl ::windows::core::RuntimeName for ILowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.ILowLevelDevicesAggregateProvider";
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl ILowLevelDevicesAggregateProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILowLevelDevicesAggregateProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILowLevelDevicesAggregateProvider_Vtbl {
        unsafe extern "system" fn AdcControllerProvider<Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PwmControllerProvider<Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GpioControllerProvider<Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn I2cControllerProvider<Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SpiControllerProvider<Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
