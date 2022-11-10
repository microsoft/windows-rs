#[cfg(feature = "Win32_Graphics_Dxgi")]
::windows_sys::core::windows_link ! ( "d3d11.dll" ,"system" fn CreateDirect3D11DeviceFromDXGIDevice ( dxgidevice : super::super::super::Graphics::Dxgi:: IDXGIDevice , graphicsdevice : *mut :: windows_sys::core::IInspectable ) -> :: windows_sys::core::HRESULT );
#[cfg(feature = "Win32_Graphics_Dxgi")]
::windows_sys::core::windows_link ! ( "d3d11.dll" ,"system" fn CreateDirect3D11SurfaceFromDXGISurface ( dgxisurface : super::super::super::Graphics::Dxgi:: IDXGISurface , graphicssurface : *mut :: windows_sys::core::IInspectable ) -> :: windows_sys::core::HRESULT );
pub type IDirect3DDxgiInterfaceAccess = *mut ::core::ffi::c_void;
