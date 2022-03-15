#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_Adc")]
pub mod Adc;
#[cfg(feature = "Devices_AllJoyn")]
pub mod AllJoyn;
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
#[cfg(feature = "Devices_Perception")]
pub mod Perception;
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
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_Adc_Provider\"`*"]
    #[cfg(feature = "Devices_Adc_Provider")]
    pub fn AdcControllerProvider(&self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdcControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Adc::Provider::IAdcControllerProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_Pwm_Provider\"`*"]
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub fn PwmControllerProvider(&self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PwmControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Pwm::Provider::IPwmControllerProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_Gpio_Provider\"`*"]
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub fn GpioControllerProvider(&self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GpioControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Gpio::Provider::IGpioControllerProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_I2c_Provider\"`*"]
    #[cfg(feature = "Devices_I2c_Provider")]
    pub fn I2cControllerProvider(&self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).I2cControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<I2c::Provider::II2cControllerProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_Spi_Provider\"`*"]
    #[cfg(feature = "Devices_Spi_Provider")]
    pub fn SpiControllerProvider(&self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SpiControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Spi::Provider::ISpiControllerProvider>(result__)
        }
    }
}
impl ::core::convert::From<ILowLevelDevicesAggregateProvider> for ::windows::core::IUnknown {
    fn from(value: ILowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILowLevelDevicesAggregateProvider> for ::windows::core::IUnknown {
    fn from(value: &ILowLevelDevicesAggregateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ILowLevelDevicesAggregateProvider> for ::windows::core::IInspectable {
    fn from(value: ILowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILowLevelDevicesAggregateProvider> for ::windows::core::IInspectable {
    fn from(value: &ILowLevelDevicesAggregateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ILowLevelDevicesAggregateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for ILowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a73e561c-aac1-4ec7-a852-479f7060d01f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ILowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa73e561c_aac1_4ec7_a852_479f7060d01f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Devices_Adc_Provider")]
    pub AdcControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Adc_Provider"))]
    AdcControllerProvider: usize,
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub PwmControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Pwm_Provider"))]
    PwmControllerProvider: usize,
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub GpioControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Gpio_Provider"))]
    GpioControllerProvider: usize,
    #[cfg(feature = "Devices_I2c_Provider")]
    pub I2cControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_I2c_Provider"))]
    I2cControllerProvider: usize,
    #[cfg(feature = "Devices_Spi_Provider")]
    pub SpiControllerProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Spi_Provider"))]
    SpiControllerProvider: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLevelDevicesAggregateProviderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLevelDevicesAggregateProviderFactory {
    type Vtable = ILowLevelDevicesAggregateProviderFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ac4aaf6_3473_465e_96d5_36281a2c57af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProviderFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adc: ::windows::core::RawPtr, pwm: ::windows::core::RawPtr, gpio: ::windows::core::RawPtr, i2c: ::windows::core::RawPtr, spi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider")))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLevelDevicesController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec23dd4_179b_45de_9b39_3ae02527de52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLevelDevicesControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILowLevelDevicesControllerStatics {
    type Vtable = ILowLevelDevicesControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x093e926a_fccb_4394_a697_19de637c2db3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DefaultProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetDefaultProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices\"`*"]
#[repr(transparent)]
pub struct LowLevelDevicesAggregateProvider(::windows::core::IUnknown);
impl LowLevelDevicesAggregateProvider {
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_Adc_Provider\"`*"]
    #[cfg(feature = "Devices_Adc_Provider")]
    pub fn AdcControllerProvider(&self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdcControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Adc::Provider::IAdcControllerProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_Pwm_Provider\"`*"]
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub fn PwmControllerProvider(&self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PwmControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Pwm::Provider::IPwmControllerProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_Gpio_Provider\"`*"]
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub fn GpioControllerProvider(&self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GpioControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Gpio::Provider::IGpioControllerProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_I2c_Provider\"`*"]
    #[cfg(feature = "Devices_I2c_Provider")]
    pub fn I2cControllerProvider(&self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).I2cControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<I2c::Provider::II2cControllerProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_Spi_Provider\"`*"]
    #[cfg(feature = "Devices_Spi_Provider")]
    pub fn SpiControllerProvider(&self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SpiControllerProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Spi::Provider::ISpiControllerProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices\"`, `\"Devices_Adc_Provider\"`, `\"Devices_Gpio_Provider\"`, `\"Devices_I2c_Provider\"`, `\"Devices_Pwm_Provider\"`, `\"Devices_Spi_Provider\"`*"]
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, Adc::Provider::IAdcControllerProvider>, Param1: ::windows::core::IntoParam<'a, Pwm::Provider::IPwmControllerProvider>, Param2: ::windows::core::IntoParam<'a, Gpio::Provider::IGpioControllerProvider>, Param3: ::windows::core::IntoParam<'a, I2c::Provider::II2cControllerProvider>, Param4: ::windows::core::IntoParam<'a, Spi::Provider::ISpiControllerProvider>>(adc: Param0, pwm: Param1, gpio: Param2, i2c: Param3, spi: Param4) -> ::windows::core::Result<LowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesAggregateProviderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), adc.into_param().abi(), pwm.into_param().abi(), gpio.into_param().abi(), i2c.into_param().abi(), spi.into_param().abi(), &mut result__).from_abi::<LowLevelDevicesAggregateProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILowLevelDevicesAggregateProviderFactory<R, F: FnOnce(&ILowLevelDevicesAggregateProviderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LowLevelDevicesAggregateProvider, ILowLevelDevicesAggregateProviderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LowLevelDevicesAggregateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesAggregateProvider;{a73e561c-aac1-4ec7-a852-479f7060d01f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_Vtbl;
    const IID: ::windows::core::GUID = <ILowLevelDevicesAggregateProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesAggregateProvider";
}
impl ::core::convert::From<LowLevelDevicesAggregateProvider> for ::windows::core::IUnknown {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLevelDevicesAggregateProvider> for ::windows::core::IUnknown {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLevelDevicesAggregateProvider> for ::windows::core::IInspectable {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLevelDevicesAggregateProvider> for ::windows::core::IInspectable {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LowLevelDevicesAggregateProvider> for ILowLevelDevicesAggregateProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: LowLevelDevicesAggregateProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LowLevelDevicesAggregateProvider> for ILowLevelDevicesAggregateProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &LowLevelDevicesAggregateProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILowLevelDevicesAggregateProvider> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ILowLevelDevicesAggregateProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILowLevelDevicesAggregateProvider> for &LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ILowLevelDevicesAggregateProvider> {
        ::core::convert::TryInto::<ILowLevelDevicesAggregateProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LowLevelDevicesAggregateProvider {}
unsafe impl ::core::marker::Sync for LowLevelDevicesAggregateProvider {}
#[doc = "*Required features: `\"Devices\"`*"]
#[repr(transparent)]
pub struct LowLevelDevicesController(::windows::core::IUnknown);
impl LowLevelDevicesController {
    #[doc = "*Required features: `\"Devices\"`*"]
    pub fn DefaultProvider() -> ::windows::core::Result<ILowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultProvider)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ILowLevelDevicesAggregateProvider>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices\"`*"]
    pub fn SetDefaultProvider<'a, Param0: ::windows::core::IntoParam<'a, ILowLevelDevicesAggregateProvider>>(value: Param0) -> ::windows::core::Result<()> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetDefaultProvider)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn ILowLevelDevicesControllerStatics<R, F: FnOnce(&ILowLevelDevicesControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LowLevelDevicesController, ILowLevelDevicesControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LowLevelDevicesController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for LowLevelDevicesController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesController;{2ec23dd4-179b-45de-9b39-3ae02527de52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_Vtbl;
    const IID: ::windows::core::GUID = <ILowLevelDevicesController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LowLevelDevicesController {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesController";
}
impl ::core::convert::From<LowLevelDevicesController> for ::windows::core::IUnknown {
    fn from(value: LowLevelDevicesController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLevelDevicesController> for ::windows::core::IUnknown {
    fn from(value: &LowLevelDevicesController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLevelDevicesController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLevelDevicesController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LowLevelDevicesController> for ::windows::core::IInspectable {
    fn from(value: LowLevelDevicesController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLevelDevicesController> for ::windows::core::IInspectable {
    fn from(value: &LowLevelDevicesController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLevelDevicesController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLevelDevicesController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LowLevelDevicesController {}
unsafe impl ::core::marker::Sync for LowLevelDevicesController {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
