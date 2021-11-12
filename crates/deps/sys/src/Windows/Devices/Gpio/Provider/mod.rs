#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GpioPinProviderValueChangedEventArgs(i32);
pub struct IGpioControllerProvider(pub *mut ::core::ffi::c_void);
pub struct IGpioPinProvider(pub *mut ::core::ffi::c_void);
pub struct IGpioPinProviderValueChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGpioPinProviderValueChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
pub struct IGpioProvider(pub *mut ::core::ffi::c_void);
pub struct ProviderGpioPinDriveMode(i32);
pub struct ProviderGpioPinEdge(i32);
pub struct ProviderGpioPinValue(i32);
pub struct ProviderGpioSharingMode(i32);
