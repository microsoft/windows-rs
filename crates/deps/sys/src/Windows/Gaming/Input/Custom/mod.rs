#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GameControllerFactoryManager(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GameControllerVersionInfo(i32);
#[repr(C)]
pub struct GipFirmwareUpdateProgress(i32);
#[repr(transparent)]
pub struct GipFirmwareUpdateResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GipFirmwareUpdateStatus(i32);
#[repr(transparent)]
pub struct GipGameControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GipMessageClass(i32);
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
#[repr(C)]
pub struct XusbDeviceSubtype(i32);
#[repr(C)]
pub struct XusbDeviceType(i32);
#[repr(transparent)]
pub struct XusbGameControllerProvider(pub *mut ::core::ffi::c_void);
