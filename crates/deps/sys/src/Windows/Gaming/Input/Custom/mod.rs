#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct GameControllerVersionInfo(i32);
#[repr(C)]
pub struct GipFirmwareUpdateProgress(i32);
#[repr(transparent)]
pub struct GipFirmwareUpdateResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GipFirmwareUpdateStatus(pub i32);
impl GipFirmwareUpdateStatus {
    pub const Completed: GipFirmwareUpdateStatus = GipFirmwareUpdateStatus(0i32);
    pub const UpToDate: GipFirmwareUpdateStatus = GipFirmwareUpdateStatus(1i32);
    pub const Failed: GipFirmwareUpdateStatus = GipFirmwareUpdateStatus(2i32);
}
#[repr(transparent)]
pub struct GipGameControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GipMessageClass(pub i32);
impl GipMessageClass {
    pub const Command: GipMessageClass = GipMessageClass(0i32);
    pub const LowLatency: GipMessageClass = GipMessageClass(1i32);
    pub const StandardLatency: GipMessageClass = GipMessageClass(2i32);
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
    pub const Unknown: XusbDeviceSubtype = XusbDeviceSubtype(0i32);
    pub const Gamepad: XusbDeviceSubtype = XusbDeviceSubtype(1i32);
    pub const ArcadePad: XusbDeviceSubtype = XusbDeviceSubtype(2i32);
    pub const ArcadeStick: XusbDeviceSubtype = XusbDeviceSubtype(3i32);
    pub const FlightStick: XusbDeviceSubtype = XusbDeviceSubtype(4i32);
    pub const Wheel: XusbDeviceSubtype = XusbDeviceSubtype(5i32);
    pub const Guitar: XusbDeviceSubtype = XusbDeviceSubtype(6i32);
    pub const GuitarAlternate: XusbDeviceSubtype = XusbDeviceSubtype(7i32);
    pub const GuitarBass: XusbDeviceSubtype = XusbDeviceSubtype(8i32);
    pub const DrumKit: XusbDeviceSubtype = XusbDeviceSubtype(9i32);
    pub const DancePad: XusbDeviceSubtype = XusbDeviceSubtype(10i32);
}
#[repr(transparent)]
pub struct XusbDeviceType(pub i32);
impl XusbDeviceType {
    pub const Unknown: XusbDeviceType = XusbDeviceType(0i32);
    pub const Gamepad: XusbDeviceType = XusbDeviceType(1i32);
}
#[repr(transparent)]
pub struct XusbGameControllerProvider(pub *mut ::core::ffi::c_void);
