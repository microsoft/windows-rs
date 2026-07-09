#[cfg(feature = "Win32_d3dcommon")]
#[inline]
pub unsafe fn D3D10CreateBlob(numbytes: usize) -> windows_core::Result<super::d3dcommon::ID3D10Blob> {
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateBlob(numbytes : usize, ppbuffer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        D3D10CreateBlob(numbytes, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
#[inline]
pub unsafe fn D3D10CreateDevice<P0>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: super::minwindef::HMODULE, flags: u32, sdkversion: u32, ppdevice: Option<*mut Option<super::d3d10::ID3D10Device>>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateDevice(padapter : *mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CreateDevice(padapter.param().abi(), drivertype, software, flags, sdkversion, ppdevice.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
#[inline]
pub unsafe fn D3D10CreateDeviceAndSwapChain<P0>(padapter: P0, drivertype: D3D10_DRIVER_TYPE, software: super::minwindef::HMODULE, flags: u32, sdkversion: u32, pswapchaindesc: Option<*const super::dxgi::DXGI_SWAP_CHAIN_DESC>, ppswapchain: Option<*mut Option<super::dxgi::IDXGISwapChain>>, ppdevice: Option<*mut Option<super::d3d10::ID3D10Device>>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d10.dll" "system" fn D3D10CreateDeviceAndSwapChain(padapter : *mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, sdkversion : u32, pswapchaindesc : *const super::dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut *mut core::ffi::c_void, ppdevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D10CreateDeviceAndSwapChain(padapter.param().abi(), drivertype, software, flags, sdkversion, pswapchaindesc.unwrap_or(core::mem::zeroed()) as _, ppswapchain.unwrap_or(core::mem::zeroed()) as _, ppdevice.unwrap_or(core::mem::zeroed()) as _) }
}
pub type D3D10_DRIVER_TYPE = i32;
pub const D3D10_DRIVER_TYPE_HARDWARE: D3D10_DRIVER_TYPE = 0;
pub const D3D10_DRIVER_TYPE_NULL: D3D10_DRIVER_TYPE = 2;
pub const D3D10_DRIVER_TYPE_REFERENCE: D3D10_DRIVER_TYPE = 1;
pub const D3D10_DRIVER_TYPE_SOFTWARE: D3D10_DRIVER_TYPE = 3;
pub const D3D10_DRIVER_TYPE_WARP: D3D10_DRIVER_TYPE = 5;
pub const GUID_DeviceType: windows_core::GUID = windows_core::GUID::from_u128(0xd722fb4d_7a68_437a_b20c_5804ee2494a6);
