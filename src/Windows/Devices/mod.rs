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
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct DevicesLowLevelContract(pub u8);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices`*"]
pub struct ILowLevelDevicesAggregateProvider(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2805880348, 43713, 20167, [168, 82, 71, 159, 112, 96, 208, 31]);
}
impl ILowLevelDevicesAggregateProvider {
    #[cfg(feature = "Devices_Adc_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_Adc_Provider`*"]
    pub fn AdcControllerProvider(&self) -> ::windows::runtime::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Adc::Provider::IAdcControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Pwm_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_Pwm_Provider`*"]
    pub fn PwmControllerProvider(&self) -> ::windows::runtime::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Pwm::Provider::IPwmControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Gpio_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_Gpio_Provider`*"]
    pub fn GpioControllerProvider(&self) -> ::windows::runtime::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Gpio::Provider::IGpioControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_I2c_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_I2c_Provider`*"]
    pub fn I2cControllerProvider(&self) -> ::windows::runtime::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<I2c::Provider::II2cControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Spi_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_Spi_Provider`*"]
    pub fn SpiControllerProvider(&self) -> ::windows::runtime::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Spi::Provider::ISpiControllerProvider>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a73e561c-aac1-4ec7-a852-479f7060d01f}");
}
impl ::std::convert::From<ILowLevelDevicesAggregateProvider> for ::windows::runtime::IUnknown {
    fn from(value: ILowLevelDevicesAggregateProvider) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ILowLevelDevicesAggregateProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ILowLevelDevicesAggregateProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ILowLevelDevicesAggregateProvider> for ::windows::runtime::IInspectable {
    fn from(value: ILowLevelDevicesAggregateProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ILowLevelDevicesAggregateProvider> for ::windows::runtime::IInspectable {
    fn from(value: &ILowLevelDevicesAggregateProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Adc_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Adc_Provider"))] usize,
    #[cfg(feature = "Devices_Pwm_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Pwm_Provider"))] usize,
    #[cfg(feature = "Devices_Gpio_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Gpio_Provider"))] usize,
    #[cfg(feature = "Devices_I2c_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_I2c_Provider"))] usize,
    #[cfg(feature = "Devices_Spi_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Spi_Provider"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProviderFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILowLevelDevicesAggregateProviderFactory {
    type Vtable = ILowLevelDevicesAggregateProviderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2596580086, 13427, 18014, [150, 213, 54, 40, 26, 44, 87, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesAggregateProviderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, adc: ::windows::runtime::RawPtr, pwm: ::windows::runtime::RawPtr, gpio: ::windows::runtime::RawPtr, i2c: ::windows::runtime::RawPtr, spi: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLevelDevicesController(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(784481748, 6043, 17886, [155, 57, 58, 224, 37, 39, 222, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILowLevelDevicesControllerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILowLevelDevicesControllerStatics {
    type Vtable = ILowLevelDevicesControllerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(155095658, 64715, 17300, [166, 151, 25, 222, 99, 124, 45, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLevelDevicesControllerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LowLevelDevicesAggregateProvider(pub ::windows::runtime::IInspectable);
impl LowLevelDevicesAggregateProvider {
    #[cfg(feature = "Devices_Adc_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_Adc_Provider`*"]
    pub fn AdcControllerProvider(&self) -> ::windows::runtime::Result<Adc::Provider::IAdcControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Adc::Provider::IAdcControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Pwm_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_Pwm_Provider`*"]
    pub fn PwmControllerProvider(&self) -> ::windows::runtime::Result<Pwm::Provider::IPwmControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Pwm::Provider::IPwmControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Gpio_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_Gpio_Provider`*"]
    pub fn GpioControllerProvider(&self) -> ::windows::runtime::Result<Gpio::Provider::IGpioControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Gpio::Provider::IGpioControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_I2c_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_I2c_Provider`*"]
    pub fn I2cControllerProvider(&self) -> ::windows::runtime::Result<I2c::Provider::II2cControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<I2c::Provider::II2cControllerProvider>(result__)
        }
    }
    #[cfg(feature = "Devices_Spi_Provider")]
    #[doc = "*Required features: `Devices`, `Devices_Spi_Provider`*"]
    pub fn SpiControllerProvider(&self) -> ::windows::runtime::Result<Spi::Provider::ISpiControllerProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Spi::Provider::ISpiControllerProvider>(result__)
        }
    }
    #[cfg(all(feature = "Devices_Adc_Provider", feature = "Devices_Gpio_Provider", feature = "Devices_I2c_Provider", feature = "Devices_Pwm_Provider", feature = "Devices_Spi_Provider"))]
    #[doc = "*Required features: `Devices`, `Devices_Adc_Provider`, `Devices_Gpio_Provider`, `Devices_I2c_Provider`, `Devices_Pwm_Provider`, `Devices_Spi_Provider`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, Adc::Provider::IAdcControllerProvider>, Param1: ::windows::runtime::IntoParam<'a, Pwm::Provider::IPwmControllerProvider>, Param2: ::windows::runtime::IntoParam<'a, Gpio::Provider::IGpioControllerProvider>, Param3: ::windows::runtime::IntoParam<'a, I2c::Provider::II2cControllerProvider>, Param4: ::windows::runtime::IntoParam<'a, Spi::Provider::ISpiControllerProvider>>(
        adc: Param0,
        pwm: Param1,
        gpio: Param2,
        i2c: Param3,
        spi: Param4,
    ) -> ::windows::runtime::Result<LowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesAggregateProviderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), adc.into_param().abi(), pwm.into_param().abi(), gpio.into_param().abi(), i2c.into_param().abi(), spi.into_param().abi(), &mut result__).from_abi::<LowLevelDevicesAggregateProvider>(result__)
        })
    }
    pub fn ILowLevelDevicesAggregateProviderFactory<R, F: FnOnce(&ILowLevelDevicesAggregateProviderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LowLevelDevicesAggregateProvider, ILowLevelDevicesAggregateProviderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LowLevelDevicesAggregateProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesAggregateProvider;{a73e561c-aac1-4ec7-a852-479f7060d01f})");
}
unsafe impl ::windows::runtime::Interface for LowLevelDevicesAggregateProvider {
    type Vtable = ILowLevelDevicesAggregateProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2805880348, 43713, 20167, [168, 82, 71, 159, 112, 96, 208, 31]);
}
impl ::windows::runtime::RuntimeName for LowLevelDevicesAggregateProvider {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesAggregateProvider";
}
impl ::std::convert::From<LowLevelDevicesAggregateProvider> for ::windows::runtime::IUnknown {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LowLevelDevicesAggregateProvider> for ::windows::runtime::IUnknown {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LowLevelDevicesAggregateProvider> for ::windows::runtime::IInspectable {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LowLevelDevicesAggregateProvider> for ::windows::runtime::IInspectable {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<LowLevelDevicesAggregateProvider> for ILowLevelDevicesAggregateProvider {
    fn from(value: LowLevelDevicesAggregateProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LowLevelDevicesAggregateProvider> for ILowLevelDevicesAggregateProvider {
    fn from(value: &LowLevelDevicesAggregateProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILowLevelDevicesAggregateProvider> for LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILowLevelDevicesAggregateProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILowLevelDevicesAggregateProvider>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILowLevelDevicesAggregateProvider> for &LowLevelDevicesAggregateProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILowLevelDevicesAggregateProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILowLevelDevicesAggregateProvider>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for LowLevelDevicesAggregateProvider {}
unsafe impl ::std::marker::Sync for LowLevelDevicesAggregateProvider {}
#[doc = "*Required features: `Devices`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LowLevelDevicesController(pub ::windows::runtime::IInspectable);
impl LowLevelDevicesController {
    #[doc = "*Required features: `Devices`*"]
    pub fn DefaultProvider() -> ::windows::runtime::Result<ILowLevelDevicesAggregateProvider> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ILowLevelDevicesAggregateProvider>(result__)
        })
    }
    #[doc = "*Required features: `Devices`*"]
    pub fn SetDefaultProvider<'a, Param0: ::windows::runtime::IntoParam<'a, ILowLevelDevicesAggregateProvider>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::ILowLevelDevicesControllerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    pub fn ILowLevelDevicesControllerStatics<R, F: FnOnce(&ILowLevelDevicesControllerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LowLevelDevicesController, ILowLevelDevicesControllerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LowLevelDevicesController {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.LowLevelDevicesController;{2ec23dd4-179b-45de-9b39-3ae02527de52})");
}
unsafe impl ::windows::runtime::Interface for LowLevelDevicesController {
    type Vtable = ILowLevelDevicesController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(784481748, 6043, 17886, [155, 57, 58, 224, 37, 39, 222, 82]);
}
impl ::windows::runtime::RuntimeName for LowLevelDevicesController {
    const NAME: &'static str = "Windows.Devices.LowLevelDevicesController";
}
impl ::std::convert::From<LowLevelDevicesController> for ::windows::runtime::IUnknown {
    fn from(value: LowLevelDevicesController) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&LowLevelDevicesController> for ::windows::runtime::IUnknown {
    fn from(value: &LowLevelDevicesController) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LowLevelDevicesController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LowLevelDevicesController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<LowLevelDevicesController> for ::windows::runtime::IInspectable {
    fn from(value: LowLevelDevicesController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LowLevelDevicesController> for ::windows::runtime::IInspectable {
    fn from(value: &LowLevelDevicesController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LowLevelDevicesController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LowLevelDevicesController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LowLevelDevicesController {}
unsafe impl ::std::marker::Sync for LowLevelDevicesController {}
