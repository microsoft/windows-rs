#[cfg(feature = "Devices_Adc")]
pub mod Adc;
#[cfg(feature = "Devices_Background")]
pub mod Background;
#[cfg(feature = "Devices_Bluetooth")]
pub mod Bluetooth;
#[cfg(feature = "Devices_Custom")]
pub mod Custom;
#[cfg(feature = "Devices_Display")]
pub mod Display;
#[cfg(feature = "Devices_Enumeration")]
pub mod Enumeration;
#[cfg(feature = "Devices_Geolocation")]
pub mod Geolocation;
#[cfg(feature = "Devices_Gpio")]
pub mod Gpio;
#[cfg(feature = "Devices_Haptics")]
pub mod Haptics;
#[cfg(feature = "Devices_HumanInterfaceDevice")]
pub mod HumanInterfaceDevice;
#[cfg(feature = "Devices_I2c")]
pub mod I2c;
#[cfg(feature = "Devices_Input")]
pub mod Input;
#[cfg(feature = "Devices_Lights")]
pub mod Lights;
#[cfg(feature = "Devices_Midi")]
pub mod Midi;
#[cfg(feature = "Devices_PointOfService")]
pub mod PointOfService;
#[cfg(feature = "Devices_Portable")]
pub mod Portable;
#[cfg(feature = "Devices_Power")]
pub mod Power;
#[cfg(feature = "Devices_Printers")]
pub mod Printers;
#[cfg(feature = "Devices_Pwm")]
pub mod Pwm;
#[cfg(feature = "Devices_Radios")]
pub mod Radios;
#[cfg(feature = "Devices_Scanners")]
pub mod Scanners;
#[cfg(feature = "Devices_Sensors")]
pub mod Sensors;
#[cfg(feature = "Devices_SerialCommunication")]
pub mod SerialCommunication;
#[cfg(feature = "Devices_SmartCards")]
pub mod SmartCards;
#[cfg(feature = "Devices_Sms")]
pub mod Sms;
#[cfg(feature = "Devices_Spi")]
pub mod Spi;
#[cfg(feature = "Devices_Usb")]
pub mod Usb;
#[cfg(feature = "Devices_WiFi")]
pub mod WiFi;
#[cfg(feature = "Devices_WiFiDirect")]
pub mod WiFiDirect;
#[doc = "*Required features: `\"Devices\"`*"]
#[repr(transparent)]
pub struct ILowLevelDevicesAggregateProvider(::windows::core::IUnknown);
impl ILowLevelDevicesAggregateProvider {
    #[doc = "*Required features: `\"Devices_Adc_Provider\"`*"]
    #[cfg(feature = "Devices_Adc_Provider")]
    pub fn AdcControllerProvider(&self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Adc::Provider::IAdcControllerProvider>();
            (::windows::core::Interface::vtable(this).AdcControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub fn PwmControllerProvider(&self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Pwm::Provider::IPwmControllerProvider>();
            (::windows::core::Interface::vtable(this).PwmControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub fn GpioControllerProvider(&self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Gpio::Provider::IGpioControllerProvider>();
            (::windows::core::Interface::vtable(this).GpioControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
    #[cfg(feature = "Devices_I2c_Provider")]
    pub fn I2cControllerProvider(&self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<I2c::Provider::II2cControllerProvider>();
            (::windows::core::Interface::vtable(this).I2cControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
    #[cfg(feature = "Devices_Spi_Provider")]
    pub fn SpiControllerProvider(&self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Spi::Provider::ISpiControllerProvider>();
            (::windows::core::Interface::vtable(this).SpiControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ILowLevelDevicesAggregateProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ILowLevelDevicesAggregateProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILowLevelDevicesAggregateProvider {}
impl ::core::fmt::Debug for ILowLevelDevicesAggregateProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILowLevelDevicesAggregateProvider").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ILowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a73e561c-aac1-4ec7-a852-479f7060d01f}");
}
unsafe impl ::windows::core::Interface for ILowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_Vtbl;
}
impl ::core::clone::Clone for ILowLevelDevicesAggregateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILowLevelDevicesAggregateProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa73e561c_aac1_4ec7_a852_479f7060d01f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Adc_Provider")]
    pub AdcControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Adc_Provider"))]
    AdcControllerProvider: usize,
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub PwmControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Pwm_Provider"))]
    PwmControllerProvider: usize,
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub GpioControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Gpio_Provider"))]
    GpioControllerProvider: usize,
    #[cfg(feature = "Devices_I2c_Provider")]
    pub I2cControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_I2c_Provider"))]
    I2cControllerProvider: usize,
    #[cfg(feature = "Devices_Spi_Provider")]
    pub SpiControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Spi_Provider"))]
    SpiControllerProvider: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLevelDevicesAggregateProviderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLevelDevicesAggregateProviderFactory {
    type Vtable = ILowLevelDevicesAggregateProviderFactory_Vtbl;
}
impl ::core::clone::Clone for ILowLevelDevicesAggregateProviderFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILowLevelDevicesAggregateProviderFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ac4aaf6_3473_465e_96d5_36281a2c57af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProviderFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adc: *mut ::core::ffi::c_void, pwm: *mut ::core::ffi::c_void, gpio: *mut ::core::ffi::c_void, i2c: *mut ::core::ffi::c_void, spi: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider")))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLevelDevicesController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_Vtbl;
}
impl ::core::clone::Clone for ILowLevelDevicesController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILowLevelDevicesController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec23dd4_179b_45de_9b39_3ae02527de52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLevelDevicesControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLevelDevicesControllerStatics {
    type Vtable = ILowLevelDevicesControllerStatics_Vtbl;
}
impl ::core::clone::Clone for ILowLevelDevicesControllerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILowLevelDevicesControllerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x093e926a_fccb_4394_a697_19de637c2db3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DefaultProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDefaultProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices\"`*"]
#[repr(transparent)]
pub struct LowLevelDevicesAggregateProvider(::windows::core::IUnknown);
impl LowLevelDevicesAggregateProvider {
    #[doc = "*Required features: `\"Devices_Adc_Provider\"`*"]
    #[cfg(feature = "Devices_Adc_Provider")]
    pub fn AdcControllerProvider(&self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Adc::Provider::IAdcControllerProvider>();
            (::windows::core::Interface::vtable(this).AdcControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub fn PwmControllerProvider(&self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Pwm::Provider::IPwmControllerProvider>();
            (::windows::core::Interface::vtable(this).PwmControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub fn GpioControllerProvider(&self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Gpio::Provider::IGpioControllerProvider>();
            (::windows::core::Interface::vtable(this).GpioControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_I2c_Provider\"`*"]
    #[cfg(feature = "Devices_I2c_Provider")]
    pub fn I2cControllerProvider(&self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<I2c::Provider::II2cControllerProvider>();
            (::windows::core::Interface::vtable(this).I2cControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Spi_Provider\"`*"]
    #[cfg(feature = "Devices_Spi_Provider")]
    pub fn SpiControllerProvider(&self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Spi::Provider::ISpiControllerProvider>();
            (::windows::core::Interface::vtable(this).SpiControllerProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Adc_Provider\"`, `\"Devices_Gpio_Provider\"`, `\"Devices_I2c_Provider\"`, `\"Devices_Pwm_Provider\"`, `\"Devices_Spi_Provider\"`*"]
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
    pub fn Create<P0, P1, P2, P3, P4>(adc: P0, pwm: P1, gpio: P2, i2c: P3, spi: P4) -> ::windows::core::Result<LowLevelDevicesAggregateProvider>
    where
        P0: ::windows::core::TryIntoParam<Adc::Provider::IAdcControllerProvider>,
        P1: ::windows::core::TryIntoParam<Pwm::Provider::IPwmControllerProvider>,
        P2: ::windows::core::TryIntoParam<Gpio::Provider::IGpioControllerProvider>,
        P3: ::windows::core::TryIntoParam<I2c::Provider::II2cControllerProvider>,
        P4: ::windows::core::TryIntoParam<Spi::Provider::ISpiControllerProvider>,
    {
        Self::ILowLevelDevicesAggregateProviderFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LowLevelDevicesAggregateProvider>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), adc.try_into_param()?.abi(), pwm.try_into_param()?.abi(), gpio.try_into_param()?.abi(), i2c.try_into_param()?.abi(), spi.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILowLevelDevicesAggregateProviderFactory<R, F: FnOnce(&ILowLevelDevicesAggregateProviderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LowLevelDevicesAggregateProvider, ILowLevelDevicesAggregateProviderFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LowLevelDevicesAggregateProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLevelDevicesAggregateProvider {}
impl ::core::fmt::Debug for LowLevelDevicesAggregateProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLevelDevicesAggregateProvider").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesAggregateProvider;{a73e561c-aac1-4ec7-a852-479f7060d01f})");
}
impl ::core::clone::Clone for LowLevelDevicesAggregateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LowLevelDevicesAggregateProvider {
    const IID: ::windows::core::GUID = <ILowLevelDevicesAggregateProvider as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesAggregateProvider";
}
::windows::imp::interface_hierarchy!(LowLevelDevicesAggregateProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<ILowLevelDevicesAggregateProvider> for LowLevelDevicesAggregateProvider {}
unsafe impl ::core::marker::Send for LowLevelDevicesAggregateProvider {}
unsafe impl ::core::marker::Sync for LowLevelDevicesAggregateProvider {}
#[doc = "*Required features: `\"Devices\"`*"]
#[repr(transparent)]
pub struct LowLevelDevicesController(::windows::core::IUnknown);
impl LowLevelDevicesController {
    pub fn DefaultProvider() -> ::windows::core::Result<ILowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ILowLevelDevicesAggregateProvider>();
            (::windows::core::Interface::vtable(this).DefaultProvider)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetDefaultProvider<P0>(value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<ILowLevelDevicesAggregateProvider>,
    {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetDefaultProvider)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ILowLevelDevicesControllerStatics<R, F: FnOnce(&ILowLevelDevicesControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LowLevelDevicesController, ILowLevelDevicesControllerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LowLevelDevicesController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLevelDevicesController {}
impl ::core::fmt::Debug for LowLevelDevicesController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLevelDevicesController").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LowLevelDevicesController {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesController;{2ec23dd4-179b-45de-9b39-3ae02527de52})");
}
impl ::core::clone::Clone for LowLevelDevicesController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LowLevelDevicesController {
    const IID: ::windows::core::GUID = <ILowLevelDevicesController as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LowLevelDevicesController {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesController";
}
::windows::imp::interface_hierarchy!(LowLevelDevicesController, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for LowLevelDevicesController {}
unsafe impl ::core::marker::Sync for LowLevelDevicesController {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
