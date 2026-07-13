#[cfg(feature = "d3d12")]
windows_link::link!("mfplat.dll" "C" fn MFCreateD3D12SynchronizationObject(pdevice : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppvsyncobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const MF_D3D11_RESOURCE: MF_MT_D3D_RESOURCE_VERSION_ENUM = 0;
pub const MF_D3D12_RESOURCE: MF_MT_D3D_RESOURCE_VERSION_ENUM = 1;
pub type MF_MT_D3D_RESOURCE_VERSION_ENUM = i32;
