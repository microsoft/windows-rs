#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ICompositionCapabilitiesInteropFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionCapabilitiesInteropFactory {}
impl ::core::clone::Clone for ICompositionCapabilitiesInteropFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionDrawingSurfaceInterop {}
impl ::core::clone::Clone for ICompositionDrawingSurfaceInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionDrawingSurfaceInterop2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionDrawingSurfaceInterop2 {}
impl ::core::clone::Clone for ICompositionDrawingSurfaceInterop2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositionGraphicsDeviceInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositionGraphicsDeviceInterop {}
impl ::core::clone::Clone for ICompositionGraphicsDeviceInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositorDesktopInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositorDesktopInterop {}
impl ::core::clone::Clone for ICompositorDesktopInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositorInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositorInterop {}
impl ::core::clone::Clone for ICompositorInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesktopWindowTargetInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesktopWindowTargetInterop {}
impl ::core::clone::Clone for IDesktopWindowTargetInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainInterop {}
impl ::core::clone::Clone for ISwapChainInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVisualInteractionSourceInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVisualInteractionSourceInterop {}
impl ::core::clone::Clone for IVisualInteractionSourceInterop {
    fn clone(&self) -> Self {
        *self
    }
}
