#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Direct3DBindings(pub u32);
impl Direct3DBindings {
    pub const VertexBuffer: Self = Self(1u32);
    pub const IndexBuffer: Self = Self(2u32);
    pub const ConstantBuffer: Self = Self(4u32);
    pub const ShaderResource: Self = Self(8u32);
    pub const StreamOutput: Self = Self(16u32);
    pub const RenderTarget: Self = Self(32u32);
    pub const DepthStencil: Self = Self(64u32);
    pub const UnorderedAccess: Self = Self(128u32);
    pub const Decoder: Self = Self(512u32);
    pub const VideoEncoder: Self = Self(1024u32);
}
#[repr(C)]
pub struct Direct3DMultisampleDescription(i32);
#[repr(C)]
pub struct Direct3DSurfaceDescription(i32);
#[repr(transparent)]
pub struct Direct3DUsage(pub i32);
impl Direct3DUsage {
    pub const Default: Self = Self(0i32);
    pub const Immutable: Self = Self(1i32);
    pub const Dynamic: Self = Self(2i32);
    pub const Staging: Self = Self(3i32);
}
#[repr(transparent)]
pub struct IDirect3DDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DSurface(pub *mut ::core::ffi::c_void);
