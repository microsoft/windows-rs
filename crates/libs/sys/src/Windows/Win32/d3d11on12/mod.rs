#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon"))]
windows_link::link!("d3d11.dll" "system" fn D3D11On12CreateDevice(pdevice : *mut core::ffi::c_void, flags : u32, pfeaturelevels : *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels : u32, ppcommandqueues : *const *mut core::ffi::c_void, numqueues : u32, nodemask : u32, ppdevice : *mut *mut core::ffi::c_void, ppimmediatecontext : *mut *mut core::ffi::c_void, pchosenfeaturelevel : *mut super::d3dcommon::D3D_FEATURE_LEVEL) -> windows_sys::core::HRESULT);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3D11_RESOURCE_FLAGS {
    pub BindFlags: u32,
    pub MiscFlags: u32,
    pub CPUAccessFlags: u32,
    pub StructureByteStride: u32,
}
#[cfg(all(feature = "Win32_d3d11", feature = "Win32_d3dcommon"))]
pub type PFN_D3D11ON12_CREATE_DEVICE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: u32, param2: *const super::d3dcommon::D3D_FEATURE_LEVEL, featurelevels: u32, param4: *const *mut core::ffi::c_void, numqueues: u32, param6: u32, param7: *mut *mut core::ffi::c_void, param8: *mut *mut core::ffi::c_void, param9: *mut super::d3dcommon::D3D_FEATURE_LEVEL) -> windows_sys::core::HRESULT>;
