#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Direct3DBindings(pub u32);
impl Direct3DBindings {
    pub const VertexBuffer: Direct3DBindings = Direct3DBindings(1u32);
    pub const IndexBuffer: Direct3DBindings = Direct3DBindings(2u32);
    pub const ConstantBuffer: Direct3DBindings = Direct3DBindings(4u32);
    pub const ShaderResource: Direct3DBindings = Direct3DBindings(8u32);
    pub const StreamOutput: Direct3DBindings = Direct3DBindings(16u32);
    pub const RenderTarget: Direct3DBindings = Direct3DBindings(32u32);
    pub const DepthStencil: Direct3DBindings = Direct3DBindings(64u32);
    pub const UnorderedAccess: Direct3DBindings = Direct3DBindings(128u32);
    pub const Decoder: Direct3DBindings = Direct3DBindings(512u32);
    pub const VideoEncoder: Direct3DBindings = Direct3DBindings(1024u32);
}
#[repr(C)]
pub struct Direct3DMultisampleDescription(i32);
#[repr(C)]
pub struct Direct3DSurfaceDescription(i32);
#[repr(transparent)]
pub struct Direct3DUsage(pub i32);
impl Direct3DUsage {
    pub const Default: Direct3DUsage = Direct3DUsage(0i32);
    pub const Immutable: Direct3DUsage = Direct3DUsage(1i32);
    pub const Dynamic: Direct3DUsage = Direct3DUsage(2i32);
    pub const Staging: Direct3DUsage = Direct3DUsage(3i32);
}
#[repr(transparent)]
pub struct IDirect3DDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DSurface(pub *mut ::core::ffi::c_void);
