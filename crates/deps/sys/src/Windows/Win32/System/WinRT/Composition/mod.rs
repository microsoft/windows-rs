#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ICompositionCapabilitiesInteropFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionGraphicsDeviceInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositorDesktopInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositorInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesktopWindowTargetInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualInteractionSourceInterop(pub *mut ::core::ffi::c_void);
