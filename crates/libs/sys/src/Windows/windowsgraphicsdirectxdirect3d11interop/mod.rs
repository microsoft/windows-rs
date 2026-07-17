#[cfg(feature = "dxgi")]
windows_link::link!("d3d11.dll" "system" fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice : *mut core::ffi::c_void, graphicsdevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "dxgi")]
windows_link::link!("d3d11.dll" "system" fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface : *mut core::ffi::c_void, graphicssurface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
