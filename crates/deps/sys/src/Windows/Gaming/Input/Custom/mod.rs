#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct GameControllerVersionInfo {
    pub Major: u16,
    pub Minor: u16,
    pub Build: u16,
    pub Revision: u16,
}
impl ::core::marker::Copy for GameControllerVersionInfo {}
impl ::core::clone::Clone for GameControllerVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GipFirmwareUpdateProgress {
    pub PercentCompleted: f64,
    pub CurrentComponentId: u32,
}
impl ::core::marker::Copy for GipFirmwareUpdateProgress {}
impl ::core::clone::Clone for GipFirmwareUpdateProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GipFirmwareUpdateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: Self = Self(0i32);
    pub const UpToDate: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct GipGameControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: Self = Self(0i32);
    pub const LowLatency: Self = Self(1i32);
    pub const StandardLatency: Self = Self(2i32);
}
#[repr(transparent)]
pub struct HidGameControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomGameControllerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameControllerFactoryManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameControllerInputSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGameControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGipFirmwareUpdateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGipGameControllerInputSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGipGameControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidGameControllerInputSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHidGameControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXusbGameControllerInputSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXusbGameControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XusbDeviceSubtype(pub i32);
impl XusbDeviceSubtype {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
    pub const ArcadePad: Self = Self(2i32);
    pub const ArcadeStick: Self = Self(3i32);
    pub const FlightStick: Self = Self(4i32);
    pub const Wheel: Self = Self(5i32);
    pub const Guitar: Self = Self(6i32);
    pub const GuitarAlternate: Self = Self(7i32);
    pub const GuitarBass: Self = Self(8i32);
    pub const DrumKit: Self = Self(9i32);
    pub const DancePad: Self = Self(10i32);
}
#[repr(transparent)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: Self = Self(0i32);
    pub const Gamepad: Self = Self(1i32);
}
#[repr(transparent)]
pub struct XusbGameControllerProvider(pub *mut ::core::ffi::c_void);
