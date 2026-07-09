#[cfg(feature = "Win32_d3dcommon")]
windows_link::link!("d3d10.dll" "system" fn D3D10CreateBlob(numbytes : usize, ppbuffer : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_dxgi", feature = "Win32_minwindef"))]
windows_link::link!("d3d10.dll" "system" fn D3D10CreateDevice(padapter : *mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_d3d10", feature = "Win32_dxgi", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("d3d10.dll" "system" fn D3D10CreateDeviceAndSwapChain(padapter : *mut core::ffi::c_void, drivertype : D3D10_DRIVER_TYPE, software : super::minwindef::HMODULE, flags : u32, sdkversion : u32, pswapchaindesc : *const super::dxgi::DXGI_SWAP_CHAIN_DESC, ppswapchain : *mut *mut core::ffi::c_void, ppdevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type D3D10_DRIVER_TYPE = i32;
pub const D3D10_DRIVER_TYPE_HARDWARE: D3D10_DRIVER_TYPE = 0;
pub const D3D10_DRIVER_TYPE_NULL: D3D10_DRIVER_TYPE = 2;
pub const D3D10_DRIVER_TYPE_REFERENCE: D3D10_DRIVER_TYPE = 1;
pub const D3D10_DRIVER_TYPE_SOFTWARE: D3D10_DRIVER_TYPE = 3;
pub const D3D10_DRIVER_TYPE_WARP: D3D10_DRIVER_TYPE = 5;
pub const GUID_DeviceType: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd722fb4d_7a68_437a_b20c_5804ee2494a6);
