#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct Direct3D11CaptureFrame(i32);
pub struct Direct3D11CaptureFramePool(i32);
pub struct GraphicsCaptureAccess(i32);
pub struct GraphicsCaptureAccessKind(i32);
pub struct GraphicsCaptureItem(i32);
pub struct GraphicsCapturePicker(i32);
pub struct GraphicsCaptureSession(i32);
pub struct IDirect3D11CaptureFrame(pub *mut ::core::ffi::c_void);
pub struct IDirect3D11CaptureFramePool(pub *mut ::core::ffi::c_void);
pub struct IDirect3D11CaptureFramePoolStatics(pub *mut ::core::ffi::c_void);
pub struct IDirect3D11CaptureFramePoolStatics2(pub *mut ::core::ffi::c_void);
pub struct IGraphicsCaptureAccessStatics(pub *mut ::core::ffi::c_void);
pub struct IGraphicsCaptureItem(pub *mut ::core::ffi::c_void);
pub struct IGraphicsCaptureItemStatics(pub *mut ::core::ffi::c_void);
pub struct IGraphicsCaptureItemStatics2(pub *mut ::core::ffi::c_void);
pub struct IGraphicsCapturePicker(pub *mut ::core::ffi::c_void);
pub struct IGraphicsCaptureSession(pub *mut ::core::ffi::c_void);
pub struct IGraphicsCaptureSession2(pub *mut ::core::ffi::c_void);
pub struct IGraphicsCaptureSession3(pub *mut ::core::ffi::c_void);
pub struct IGraphicsCaptureSessionStatics(pub *mut ::core::ffi::c_void);
