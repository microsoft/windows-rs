#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: super::super::super::Graphics::Dxgi::IDXGIDevice, graphicsdevice: *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: super::super::super::Graphics::Dxgi::IDXGISurface, graphicssurface: *mut ::windows_sys::core::IInspectable) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct IDirect3DDxgiInterfaceAccess(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDirect3DDxgiInterfaceAccess {}
impl ::core::clone::Clone for IDirect3DDxgiInterfaceAccess {
    fn clone(&self) -> Self {
        *self
    }
}
