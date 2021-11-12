#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Gpio_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct GpioChangeCount(i32);
#[repr(transparent)]
pub struct GpioChangeCounter(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GpioChangePolarity(i32);
#[repr(transparent)]
pub struct GpioChangeReader(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct GpioChangeRecord(i32);
#[repr(transparent)]
pub struct GpioController(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GpioOpenStatus(i32);
#[repr(transparent)]
pub struct GpioPin(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GpioPinDriveMode(i32);
#[repr(C)]
pub struct GpioPinEdge(i32);
#[repr(C)]
pub struct GpioPinValue(i32);
#[repr(transparent)]
pub struct GpioPinValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GpioSharingMode(i32);
#[repr(transparent)]
pub struct IGpioChangeCounter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioChangeCounterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioChangeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioChangeReaderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioControllerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioPin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioPinValueChangedEventArgs(pub *mut ::core::ffi::c_void);
