#[doc = "*Required features: `\"Devices\"`, `\"Devices_Adc_Provider\"`, `\"Devices_Gpio_Provider\"`, `\"Devices_I2c_Provider\"`, `\"Devices_Pwm_Provider\"`, `\"Devices_Spi_Provider\"`, `\"implement\"`*"]
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
pub trait ILowLevelDevicesAggregateProvider_Impl: Sized {
    fn AdcControllerProvider(&self) -> ::windows_core::Result<Adc::Provider::IAdcControllerProvider>;
    fn PwmControllerProvider(&self) -> ::windows_core::Result<Pwm::Provider::IPwmControllerProvider>;
    fn GpioControllerProvider(&self) -> ::windows_core::Result<Gpio::Provider::IGpioControllerProvider>;
    fn I2cControllerProvider(&self) -> ::windows_core::Result<I2c::Provider::II2cControllerProvider>;
    fn SpiControllerProvider(&self) -> ::windows_core::Result<Spi::Provider::ISpiControllerProvider>;
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl ::windows_core::RuntimeName for ILowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.ILowLevelDevicesAggregateProvider";
}
#[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
impl ILowLevelDevicesAggregateProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>() -> ILowLevelDevicesAggregateProvider_Vtbl {
        unsafe extern "system" fn AdcControllerProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdcControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PwmControllerProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PwmControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GpioControllerProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GpioControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn I2cControllerProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.I2cControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpiControllerProvider<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILowLevelDevicesAggregateProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpiControllerProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ILowLevelDevicesAggregateProvider, OFFSET>(),
            AdcControllerProvider: AdcControllerProvider::<Identity, Impl, OFFSET>,
            PwmControllerProvider: PwmControllerProvider::<Identity, Impl, OFFSET>,
            GpioControllerProvider: GpioControllerProvider::<Identity, Impl, OFFSET>,
            I2cControllerProvider: I2cControllerProvider::<Identity, Impl, OFFSET>,
            SpiControllerProvider: SpiControllerProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ILowLevelDevicesAggregateProvider as ::windows_core::ComInterface>::IID
    }
}
