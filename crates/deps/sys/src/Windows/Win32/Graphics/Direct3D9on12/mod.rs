#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut D3D9ON12_ARGS, numoverrideentries: u32, ppoutputinterface: *mut super::Direct3D9::IDirect3D9Ex) -> ::windows_sys::core::HRESULT;
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct D3D9ON12_ARGS(i32);
#[repr(transparent)]
pub struct IDirect3DDevice9On12(pub *mut ::core::ffi::c_void);
pub const MAX_D3D9ON12_QUEUES: u32 = 2u32;
#[repr(C)]
pub struct PFN_Direct3DCreate9On12(i32);
#[repr(C)]
pub struct PFN_Direct3DCreate9On12Ex(i32);
