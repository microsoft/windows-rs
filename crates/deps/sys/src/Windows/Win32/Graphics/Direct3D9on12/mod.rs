#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn Direct3DCreate9On12(sdkversion: u32, poverridelist: *mut ::core::mem::ManuallyDrop<D3D9ON12_ARGS>, numoverrideentries: u32) -> ::core::option::Option<super::Direct3D9::IDirect3D9>;
    #[doc = "*Required features: `Win32_Graphics_Direct3D9on12`, `Win32_Foundation`, `Win32_Graphics_Direct3D9`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub fn Direct3DCreate9On12Ex(sdkversion: u32, poverridelist: *mut ::core::mem::ManuallyDrop<D3D9ON12_ARGS>, numoverrideentries: u32, ppoutputinterface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
