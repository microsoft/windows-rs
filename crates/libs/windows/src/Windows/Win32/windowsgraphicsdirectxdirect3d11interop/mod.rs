#[cfg(feature = "dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11DeviceFromDXGIDevice<P0>(dxgidevice: P0) -> windows_core::Result<windows_core::IInspectable>
where
    P0: windows_core::Param<super::IDXGIDevice>,
{
    windows_core::link!("d3d11.dll" "system" fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice : *mut core::ffi::c_void, graphicsdevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateDirect3D11DeviceFromDXGIDevice(dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11SurfaceFromDXGISurface<P0>(dgxisurface: P0) -> windows_core::Result<windows_core::IInspectable>
where
    P0: windows_core::Param<super::IDXGISurface>,
{
    windows_core::link!("d3d11.dll" "system" fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface : *mut core::ffi::c_void, graphicssurface : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateDirect3D11SurfaceFromDXGISurface(dgxisurface.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
