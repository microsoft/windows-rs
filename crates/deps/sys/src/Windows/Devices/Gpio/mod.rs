#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Gpio_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct GpioChangeCount(i32);
#[repr(transparent)]
pub struct GpioChangeCounter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GpioChangePolarity(pub i32);
impl GpioChangePolarity {
    pub const Falling: GpioChangePolarity = GpioChangePolarity(0i32);
    pub const Rising: GpioChangePolarity = GpioChangePolarity(1i32);
    pub const Both: GpioChangePolarity = GpioChangePolarity(2i32);
}
#[repr(transparent)]
pub struct GpioChangeReader(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Foundation")]
#[repr(C)]
pub struct GpioChangeRecord(i32);
#[repr(transparent)]
pub struct GpioController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GpioOpenStatus(pub i32);
impl GpioOpenStatus {
    pub const PinOpened: GpioOpenStatus = GpioOpenStatus(0i32);
    pub const PinUnavailable: GpioOpenStatus = GpioOpenStatus(1i32);
    pub const SharingViolation: GpioOpenStatus = GpioOpenStatus(2i32);
    pub const MuxingConflict: GpioOpenStatus = GpioOpenStatus(3i32);
    pub const UnknownError: GpioOpenStatus = GpioOpenStatus(4i32);
}
#[repr(transparent)]
pub struct GpioPin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GpioPinDriveMode(pub i32);
impl GpioPinDriveMode {
    pub const Input: GpioPinDriveMode = GpioPinDriveMode(0i32);
    pub const Output: GpioPinDriveMode = GpioPinDriveMode(1i32);
    pub const InputPullUp: GpioPinDriveMode = GpioPinDriveMode(2i32);
    pub const InputPullDown: GpioPinDriveMode = GpioPinDriveMode(3i32);
    pub const OutputOpenDrain: GpioPinDriveMode = GpioPinDriveMode(4i32);
    pub const OutputOpenDrainPullUp: GpioPinDriveMode = GpioPinDriveMode(5i32);
    pub const OutputOpenSource: GpioPinDriveMode = GpioPinDriveMode(6i32);
    pub const OutputOpenSourcePullDown: GpioPinDriveMode = GpioPinDriveMode(7i32);
}
#[repr(transparent)]
pub struct GpioPinEdge(pub i32);
impl GpioPinEdge {
    pub const FallingEdge: GpioPinEdge = GpioPinEdge(0i32);
    pub const RisingEdge: GpioPinEdge = GpioPinEdge(1i32);
}
#[repr(transparent)]
pub struct GpioPinValue(pub i32);
impl GpioPinValue {
    pub const Low: GpioPinValue = GpioPinValue(0i32);
    pub const High: GpioPinValue = GpioPinValue(1i32);
}
#[repr(transparent)]
pub struct GpioPinValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GpioSharingMode(pub i32);
impl GpioSharingMode {
    pub const Exclusive: GpioSharingMode = GpioSharingMode(0i32);
    pub const SharedReadOnly: GpioSharingMode = GpioSharingMode(1i32);
}
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
