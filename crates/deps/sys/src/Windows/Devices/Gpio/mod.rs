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
    pub const Falling: Self = Self(0i32);
    pub const Rising: Self = Self(1i32);
    pub const Both: Self = Self(2i32);
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
    pub const PinOpened: Self = Self(0i32);
    pub const PinUnavailable: Self = Self(1i32);
    pub const SharingViolation: Self = Self(2i32);
    pub const MuxingConflict: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
#[repr(transparent)]
pub struct GpioPin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GpioPinDriveMode(pub i32);
impl GpioPinDriveMode {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const InputPullUp: Self = Self(2i32);
    pub const InputPullDown: Self = Self(3i32);
    pub const OutputOpenDrain: Self = Self(4i32);
    pub const OutputOpenDrainPullUp: Self = Self(5i32);
    pub const OutputOpenSource: Self = Self(6i32);
    pub const OutputOpenSourcePullDown: Self = Self(7i32);
}
#[repr(transparent)]
pub struct GpioPinEdge(pub i32);
impl GpioPinEdge {
    pub const FallingEdge: Self = Self(0i32);
    pub const RisingEdge: Self = Self(1i32);
}
#[repr(transparent)]
pub struct GpioPinValue(pub i32);
impl GpioPinValue {
    pub const Low: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
#[repr(transparent)]
pub struct GpioPinValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GpioSharingMode(pub i32);
impl GpioSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
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
