#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GpioPinProviderValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioPinProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioPinProviderValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioPinProviderValueChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGpioProvider(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ProviderGpioPinDriveMode(i32);
#[repr(C)]
pub struct ProviderGpioPinEdge(i32);
#[repr(C)]
pub struct ProviderGpioPinValue(i32);
#[repr(C)]
pub struct ProviderGpioSharingMode(i32);
