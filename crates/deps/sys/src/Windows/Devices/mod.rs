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
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILowLevelDevicesAggregateProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLevelDevicesAggregateProvider {}
impl ::core::clone::Clone for ILowLevelDevicesAggregateProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLevelDevicesAggregateProviderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLevelDevicesAggregateProviderFactory {}
impl ::core::clone::Clone for ILowLevelDevicesAggregateProviderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLevelDevicesController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLevelDevicesController {}
impl ::core::clone::Clone for ILowLevelDevicesController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILowLevelDevicesControllerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILowLevelDevicesControllerStatics {}
impl ::core::clone::Clone for ILowLevelDevicesControllerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LowLevelDevicesAggregateProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LowLevelDevicesAggregateProvider {}
impl ::core::clone::Clone for LowLevelDevicesAggregateProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LowLevelDevicesController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LowLevelDevicesController {}
impl ::core::clone::Clone for LowLevelDevicesController {
    fn clone(&self) -> Self {
        *self
    }
}
