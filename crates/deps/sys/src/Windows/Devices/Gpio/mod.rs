#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Gpio_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[cfg(feature = "Foundation")]
pub struct GpioChangeCount(i32);
pub struct GpioChangeCounter(i32);
pub struct GpioChangePolarity(i32);
pub struct GpioChangeReader(i32);
#[cfg(feature = "Foundation")]
pub struct GpioChangeRecord(i32);
pub struct GpioController(i32);
pub struct GpioOpenStatus(i32);
pub struct GpioPin(i32);
pub struct GpioPinDriveMode(i32);
pub struct GpioPinEdge(i32);
pub struct GpioPinValue(i32);
pub struct GpioPinValueChangedEventArgs(i32);
pub struct GpioSharingMode(i32);
pub struct IGpioChangeCounter(pub *mut ::core::ffi::c_void);
pub struct IGpioChangeCounterFactory(pub *mut ::core::ffi::c_void);
pub struct IGpioChangeReader(pub *mut ::core::ffi::c_void);
pub struct IGpioChangeReaderFactory(pub *mut ::core::ffi::c_void);
pub struct IGpioController(pub *mut ::core::ffi::c_void);
pub struct IGpioControllerStatics(pub *mut ::core::ffi::c_void);
pub struct IGpioControllerStatics2(pub *mut ::core::ffi::c_void);
pub struct IGpioPin(pub *mut ::core::ffi::c_void);
pub struct IGpioPinValueChangedEventArgs(pub *mut ::core::ffi::c_void);
