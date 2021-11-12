#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GameControllerFactoryManager(i32);
pub struct GameControllerVersionInfo(i32);
pub struct GipFirmwareUpdateProgress(i32);
pub struct GipFirmwareUpdateResult(i32);
pub struct GipFirmwareUpdateStatus(i32);
pub struct GipGameControllerProvider(i32);
pub struct GipMessageClass(i32);
pub struct HidGameControllerProvider(i32);
pub struct ICustomGameControllerFactory(pub *mut ::core::ffi::c_void);
pub struct IGameControllerFactoryManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IGameControllerFactoryManagerStatics2(pub *mut ::core::ffi::c_void);
pub struct IGameControllerInputSink(pub *mut ::core::ffi::c_void);
pub struct IGameControllerProvider(pub *mut ::core::ffi::c_void);
pub struct IGipFirmwareUpdateResult(pub *mut ::core::ffi::c_void);
pub struct IGipGameControllerInputSink(pub *mut ::core::ffi::c_void);
pub struct IGipGameControllerProvider(pub *mut ::core::ffi::c_void);
pub struct IHidGameControllerInputSink(pub *mut ::core::ffi::c_void);
pub struct IHidGameControllerProvider(pub *mut ::core::ffi::c_void);
pub struct IXusbGameControllerInputSink(pub *mut ::core::ffi::c_void);
pub struct IXusbGameControllerProvider(pub *mut ::core::ffi::c_void);
pub struct XusbDeviceSubtype(i32);
pub struct XusbDeviceType(i32);
pub struct XusbGameControllerProvider(i32);
