#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Direct3D11CaptureFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Direct3D11CaptureFrame {}
impl ::core::clone::Clone for Direct3D11CaptureFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Direct3D11CaptureFramePool(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Direct3D11CaptureFramePool {}
impl ::core::clone::Clone for Direct3D11CaptureFramePool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GraphicsCaptureAccessKind(pub i32);
impl GraphicsCaptureAccessKind {
    pub const Borderless: Self = Self(0i32);
    pub const Programmatic: Self = Self(1i32);
}
impl ::core::marker::Copy for GraphicsCaptureAccessKind {}
impl ::core::clone::Clone for GraphicsCaptureAccessKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GraphicsCaptureItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GraphicsCaptureItem {}
impl ::core::clone::Clone for GraphicsCaptureItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GraphicsCapturePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GraphicsCapturePicker {}
impl ::core::clone::Clone for GraphicsCapturePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GraphicsCaptureSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GraphicsCaptureSession {}
impl ::core::clone::Clone for GraphicsCaptureSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3D11CaptureFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3D11CaptureFrame {}
impl ::core::clone::Clone for IDirect3D11CaptureFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePool(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3D11CaptureFramePool {}
impl ::core::clone::Clone for IDirect3D11CaptureFramePool {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3D11CaptureFramePoolStatics {}
impl ::core::clone::Clone for IDirect3D11CaptureFramePoolStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDirect3D11CaptureFramePoolStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3D11CaptureFramePoolStatics2 {}
impl ::core::clone::Clone for IDirect3D11CaptureFramePoolStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsCaptureAccessStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsCaptureAccessStatics {}
impl ::core::clone::Clone for IGraphicsCaptureAccessStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsCaptureItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsCaptureItem {}
impl ::core::clone::Clone for IGraphicsCaptureItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsCaptureItemStatics {}
impl ::core::clone::Clone for IGraphicsCaptureItemStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsCaptureItemStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsCaptureItemStatics2 {}
impl ::core::clone::Clone for IGraphicsCaptureItemStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsCapturePicker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsCapturePicker {}
impl ::core::clone::Clone for IGraphicsCapturePicker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsCaptureSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsCaptureSession {}
impl ::core::clone::Clone for IGraphicsCaptureSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsCaptureSession2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsCaptureSession2 {}
impl ::core::clone::Clone for IGraphicsCaptureSession2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsCaptureSession3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsCaptureSession3 {}
impl ::core::clone::Clone for IGraphicsCaptureSession3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsCaptureSessionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsCaptureSessionStatics {}
impl ::core::clone::Clone for IGraphicsCaptureSessionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
