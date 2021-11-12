#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
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
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ILowLevelDevicesAggregateProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa73e561c_aac1_4ec7_a852_479f7060d01f);
}
impl ILowLevelDevicesAggregateProvider {
    #[cfg(feature = "Devices_Adc_Provider")]
    pub fn AdcControllerProvider(&self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Adc::Provider::IAdcControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub fn PwmControllerProvider(&self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Pwm::Provider::IPwmControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub fn GpioControllerProvider(&self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Gpio::Provider::IGpioControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_I2c_Provider")]
    pub fn I2cControllerProvider(&self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<I2c::Provider::II2cControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Spi_Provider")]
    pub fn SpiControllerProvider(&self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Spi::Provider::ISpiControllerProvider>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ILowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a73e561c-aac1-4ec7-a852-479f7060d01f}");
}
impl ::core::convert::From<ILowLevelDevicesAggregateProvider> for ::windows::core::IUnknown {
    fn from(value: ILowLevelDevicesAggregateProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILowLevelDevicesAggregateProvider> for ::windows::core::IUnknown {
    fn from(value: &ILowLevelDevicesAggregateProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILowLevelDevicesAggregateProvider> for ::windows::core::IInspectable {
    fn from(value: ILowLevelDevicesAggregateProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILowLevelDevicesAggregateProvider> for ::windows::core::IInspectable {
    fn from(value: &ILowLevelDevicesAggregateProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Adc_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Adc_Provider"))] usize,
    #[cfg(feature = "Devices_Pwm_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Pwm_Provider"))] usize,
    #[cfg(feature = "Devices_Gpio_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Gpio_Provider"))] usize,
    #[cfg(feature = "Devices_I2c_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_I2c_Provider"))] usize,
    #[cfg(feature = "Devices_Spi_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Spi_Provider"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProviderFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLevelDevicesAggregateProviderFactory {
    type Vtable = ILowLevelDevicesAggregateProviderFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ac4aaf6_3473_465e_96d5_36281a2c57af);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProviderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, adc: ::windows::core::RawPtr, pwm: ::windows::core::RawPtr, gpio: ::windows::core::RawPtr, i2c: ::windows::core::RawPtr, spi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLevelDevicesController(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec23dd4_179b_45de_9b39_3ae02527de52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesController_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLevelDevicesControllerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILowLevelDevicesControllerStatics {
    type Vtable = ILowLevelDevicesControllerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x093e926a_fccb_4394_a697_19de637c2db3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LowLevelDevicesAggregateProvider(pub ::windows::core::IInspectable);
impl LowLevelDevicesAggregateProvider {
    #[cfg(feature = "Devices_Adc_Provider")]
    pub fn AdcControllerProvider(&self) -> ::windows::core::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Adc::Provider::IAdcControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Pwm_Provider")]
    pub fn PwmControllerProvider(&self) -> ::windows::core::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Pwm::Provider::IPwmControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Gpio_Provider")]
    pub fn GpioControllerProvider(&self) -> ::windows::core::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Gpio::Provider::IGpioControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_I2c_Provider")]
    pub fn I2cControllerProvider(&self) -> ::windows::core::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<I2c::Provider::II2cControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Spi_Provider")]
    pub fn SpiControllerProvider(&self) -> ::windows::core::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Spi::Provider::ISpiControllerProvider>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, Adc::Provider::IAdcControllerProvider>, Param1: ::windows::core::IntoParam<'a, Pwm::Provider::IPwmControllerProvider>, Param2: ::windows::core::IntoParam<'a, Gpio::Provider::IGpioControllerProvider>, Param3: ::windows::core::IntoParam<'a, I2c::Provider::II2cControllerProvider>, Param4: ::windows::core::IntoParam<'a, Spi::Provider::ISpiControllerProvider>>(
        adc: Param0,
        pwm: Param1,
        gpio: Param2,
        i2c: Param3,
        spi: Param4,
    ) -> ::windows::core::Result<LowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesAggregateProviderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), adc.into_param().abi(), pwm.into_param().abi(), gpio.into_param().abi(), i2c.into_param().abi(), spi.into_param().abi(), &mut result__).from_abi::<LowLevelDevicesAggregateProvider>(result__)
        })
    }
    pub fn ILowLevelDevicesAggregateProviderFactory<R, F: FnOnce(&ILowLevelDevicesAggregateProviderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LowLevelDevicesAggregateProvider, ILowLevelDevicesAggregateProviderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesAggregateProvider;{a73e561c-aac1-4ec7-a852-479f7060d01f})");
}
unsafe impl ::windows::core::Interface for LowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa73e561c_aac1_4ec7_a852_479f7060d01f);
}
impl ::windows::core::RuntimeName for LowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesAggregateProvider";
}
impl ::core::convert::From<LowLevelDevicesAggregateProvider> for ::windows::core::IUnknown {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LowLevelDevicesAggregateProvider> for ::windows::core::IUnknown {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LowLevelDevicesAggregateProvider> for ::windows::core::IInspectable {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LowLevelDevicesAggregateProvider> for ::windows::core::IInspectable {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LowLevelDevicesAggregateProvider> for ILowLevelDevicesAggregateProvider {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LowLevelDevicesAggregateProvider> for ILowLevelDevicesAggregateProvider {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILowLevelDevicesAggregateProvider> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ILowLevelDevicesAggregateProvider> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILowLevelDevicesAggregateProvider> for &LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ILowLevelDevicesAggregateProvider> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LowLevelDevicesAggregateProvider {}
unsafe impl ::core::marker::Sync for LowLevelDevicesAggregateProvider {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LowLevelDevicesController(pub ::windows::core::IInspectable);
impl LowLevelDevicesController {
    pub fn DefaultProvider() -> ::windows::core::Result<ILowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ILowLevelDevicesAggregateProvider>(result__)
        })
    }
    pub fn SetDefaultProvider<'a, Param0: ::windows::core::IntoParam<'a, ILowLevelDevicesAggregateProvider>>(value: Param0) -> ::windows::core::Result<()> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    pub fn ILowLevelDevicesControllerStatics<R, F: FnOnce(&ILowLevelDevicesControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LowLevelDevicesController, ILowLevelDevicesControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LowLevelDevicesController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesController;{2ec23dd4-179b-45de-9b39-3ae02527de52})");
}
unsafe impl ::windows::core::Interface for LowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ec23dd4_179b_45de_9b39_3ae02527de52);
}
impl ::windows::core::RuntimeName for LowLevelDevicesController {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesController";
}
impl ::core::convert::From<LowLevelDevicesController> for ::windows::core::IUnknown {
    fn from(value: LowLevelDevicesController) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LowLevelDevicesController> for ::windows::core::IUnknown {
    fn from(value: &LowLevelDevicesController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LowLevelDevicesController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LowLevelDevicesController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LowLevelDevicesController> for ::windows::core::IInspectable {
    fn from(value: LowLevelDevicesController) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LowLevelDevicesController> for ::windows::core::IInspectable {
    fn from(value: &LowLevelDevicesController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LowLevelDevicesController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LowLevelDevicesController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LowLevelDevicesController {}
unsafe impl ::core::marker::Sync for LowLevelDevicesController {}
