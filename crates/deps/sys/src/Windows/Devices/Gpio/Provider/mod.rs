#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct ProviderGpioPinDriveMode(pub i32);
impl ProviderGpioPinDriveMode {
    pub const Input: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(0i32);
    pub const Output: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(1i32);
    pub const InputPullUp: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(2i32);
    pub const InputPullDown: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(3i32);
    pub const OutputOpenDrain: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(4i32);
    pub const OutputOpenDrainPullUp: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(5i32);
    pub const OutputOpenSource: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(6i32);
    pub const OutputOpenSourcePullDown: ProviderGpioPinDriveMode = ProviderGpioPinDriveMode(7i32);
}
#[repr(transparent)]
pub struct ProviderGpioPinEdge(pub i32);
impl ProviderGpioPinEdge {
    pub const FallingEdge: ProviderGpioPinEdge = ProviderGpioPinEdge(0i32);
    pub const RisingEdge: ProviderGpioPinEdge = ProviderGpioPinEdge(1i32);
}
#[repr(transparent)]
pub struct ProviderGpioPinValue(pub i32);
impl ProviderGpioPinValue {
    pub const Low: ProviderGpioPinValue = ProviderGpioPinValue(0i32);
    pub const High: ProviderGpioPinValue = ProviderGpioPinValue(1i32);
}
#[repr(transparent)]
pub struct ProviderGpioSharingMode(pub i32);
impl ProviderGpioSharingMode {
    pub const Exclusive: ProviderGpioSharingMode = ProviderGpioSharingMode(0i32);
    pub const SharedReadOnly: ProviderGpioSharingMode = ProviderGpioSharingMode(1i32);
}
