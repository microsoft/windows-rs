#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
impl Default for D3D11_RESOURCE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for D3D11_RESOURCE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Direct3D11"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = Option<unsafe extern "system" fn(param0: Option<windows_core::IUnknown>, param1: u32, param2: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const Option<windows_core::IUnknown>, numqueues: u32, param6: u32, param7: *mut Option<super::Direct3D11::ID3D11Device>, param8: *mut Option<super::Direct3D11::ID3D11DeviceContext>, param9: *mut super::Direct3D::D3D_FEATURE_LEVEL) -> windows_core::HRESULT>;
