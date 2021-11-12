#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Direct3D11CaptureFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Direct3D11CaptureFramePool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GraphicsCaptureAccess(pub *mut ::core::ffi::c_void);
pub struct GraphicsCaptureAccessKind(i32);
#[repr(transparent)]
pub struct GraphicsCaptureItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GraphicsCapturePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GraphicsCaptureSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3D11CaptureFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePool(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsCaptureAccessStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsCaptureItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsCapturePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsCaptureSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsCaptureSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsCaptureSession3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsCaptureSessionStatics(pub *mut ::core::ffi::c_void);
