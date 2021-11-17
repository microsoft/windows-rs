#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
    pub fn D3D11On12CreateDevice(
        pdevice: ::core::option::Option<::windows_sys::core::IUnknown>,
        flags: u32,
        pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL,
        featurelevels: u32,
        ppcommandqueues: *const ::core::option::Option<::windows_sys::core::IUnknown>,
        numqueues: u32,
        nodemask: u32,
        ppdevice: *mut ::core::option::Option<super::Direct3D11::ID3D11Device>,
        ppimmediatecontext: *mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>,
        pchosenfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL,
    ) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
impl ::core::marker::Copy for D3D11_RESOURCE_FLAGS {}
impl ::core::clone::Clone for D3D11_RESOURCE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type ID3D11On12Device = *mut ::core::ffi::c_void;
pub type ID3D11On12Device1 = *mut ::core::ffi::c_void;
pub type ID3D11On12Device2 = *mut ::core::ffi::c_void;
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = unsafe extern "system" fn(
    param0: ::core::option::Option<::windows_sys::core::IUnknown>,
    param1: u32,
    param2: *const super::Direct3D::D3D_FEATURE_LEVEL,
    featurelevels: u32,
    param4: *const ::core::option::Option<::windows_sys::core::IUnknown>,
    numqueues: u32,
    param6: u32,
    param7: *mut ::core::option::Option<super::Direct3D11::ID3D11Device>,
    param8: *mut ::core::option::Option<super::Direct3D11::ID3D11DeviceContext>,
    param9: *mut super::Direct3D::D3D_FEATURE_LEVEL,
) -> ::windows_sys::core::HRESULT;
